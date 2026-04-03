// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License. You may obtain a copy of
// the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under
// the License.

#![no_std]
#![no_main]

use app_usb_tpm_bridge::{handle, signals};
use pw_status::Result;
use userspace::time::Instant;
use userspace::{entry, syscall};

use aligned::{Aligned, A4};
use hal_usb::{
    ConfigDescriptor, DeviceDescriptor, StringDescriptorRef, StringHandle,
};

use hal_usb::driver::UsbDriver;
use usb_driver::UsbConfig;
use usb_stack::{DescriptorSource, UsbAction, UsbClass};

use protocol_usb_usbtpm::{UsbTpm, UsbTpmBuilder, TpmDevice};

const USB_VENDOR_HANDLE: StringHandle = StringHandle(1);
const USB_PRODUCT_HANDLE: StringHandle = StringHandle(2);
const USB_SERIAL_HANDLE: StringHandle = StringHandle(3);
const USB_TPM_IF_HANDLE: StringHandle = StringHandle(4);

const TPM_BUILDER: UsbTpmBuilder = UsbTpmBuilder::new(
    0, // interface index
    1, // bulk in endpoint
    1, // bulk out endpoint
);

const DEVICE_DESC: DeviceDescriptor = DeviceDescriptor {
    device_class: hal_usb::DeviceClass::SPECIFIED_BY_INTERFACE,
    device_sub_class: 0x00,
    device_protocol: 0x00,
    max_packet_size: 64,
    vendor_id: 0x18d1,
    product_id: 0x5031, // GSC TPM
    device_release_num: 0x0100,
    manufacturer: USB_VENDOR_HANDLE,
    product: USB_PRODUCT_HANDLE,
    serial_num: USB_SERIAL_HANDLE,
};

const CONFIG_DESC: ConfigDescriptor = ConfigDescriptor {
    configuration_value: 1,
    max_power: 250,
    self_powered: false,
    remote_wakeup: false,
    interfaces: &[
        TPM_BUILDER.interface(USB_TPM_IF_HANDLE, &TPM_BUILDER.endpoints()),
    ],
};

const STRING_DESC_0: hal_usb::StringDescriptor0 = hal_usb::StringDescriptor0 {
    langs: &[0x0409], // English - US
};

const VENDOR_ID: StringDescriptorRef = hal_usb::string_descriptor!("Google Inc.").as_ref();
const PRODUCT_ID: StringDescriptorRef = hal_usb::string_descriptor!("OpenPRoT TPM").as_ref();
const TPM_IF_NAME: StringDescriptorRef = hal_usb::string_descriptor!("TPM Interface").as_ref();

struct MyDescriptors<'a> {
    serial_desc_bytes: StringDescriptorRef<'a>,
}

impl DescriptorSource for MyDescriptors<'_> {
    const DEVICE_DESC_BYTES: &'static Aligned<A4, [u8]> = &Aligned(DEVICE_DESC.serialize());
    const CONFIG_DESC_BYTES: &'static Aligned<A4, [u8]> =
        &Aligned(CONFIG_DESC.serialize::<{ CONFIG_DESC.total_size() }>());
    const STRING_DESC_0_BYTES: &'static Aligned<A4, [u8]> =
        &Aligned(STRING_DESC_0.serialize::<{ STRING_DESC_0.total_size() }>());
    const DEVICE_STATUS: Aligned<A4, [u8; 2]> = Aligned([1u8, 0]);

    fn get_string(&self, handle: StringHandle, _lang: u16) -> Option<StringDescriptorRef<'_>> {
        match handle {
            USB_VENDOR_HANDLE => Some(VENDOR_ID),
            USB_PRODUCT_HANDLE => Some(PRODUCT_ID),
            USB_SERIAL_HANDLE => Some(self.serial_desc_bytes),
            USB_TPM_IF_HANDLE => Some(TPM_IF_NAME),
            _ => None,
        }
    }
}

struct IpcTpmDevice {
    ipc_handle: u32,
}

impl TpmDevice for IpcTpmDevice {
    fn execute_command(&mut self, _locality: u8, command: &[u8], response: &mut [u8]) -> usize {
        // Forward the command to the tpm_service process via IPC.
        match syscall::channel_transact(self.ipc_handle, command, response, Instant::MAX) {
            Ok(len) => len,
            Err(e) => {
                pw_log::error!("TPM Bridge: IPC transaction failed: {}", e as u32);
                0
            }
        }
    }

    fn cancel(&mut self) {
        // Cancel not yet implemented over IPC.
    }
}

fn handle_usb() -> Result<()> {
    let mut serial_num_buffer = Aligned::<A4, _>([0_u8; 130]);
    let descriptors = MyDescriptors {
        serial_desc_bytes: hal_usb::hex_utf16_descriptor_aligned(&mut serial_num_buffer, b"TPM-BRIDGE-01")
            .unwrap(),
    };

    const USB_CONFIG: UsbConfig = UsbConfig::new(&TPM_BUILDER.eps().0, &TPM_BUILDER.eps().1);

    let mut usb = usb_driver::Usb::new(unsafe { usbdev::Usbdev::new() }, USB_CONFIG);
    let mut ep0 = usb_stack::SimpleEp0::new();
    
    let mut tpm_device = IpcTpmDevice {
        ipc_handle: handle::IPC,
    };
    // 1024 words = 4KB payload capacity.
    let mut usbtpm = UsbTpm::<_, 1024>::new(TPM_BUILDER, &mut tpm_device);

    loop {
        let _wait_return = syscall::object_wait(
            handle::USBDEV_INTERRUPTS,
            signals::USBDEV_PKT_RECEIVED
                | signals::USBDEV_PKT_SENT
                | signals::USBDEV_DISCONNECTED
                | signals::USBDEV_LINK_RESET
                | signals::USBDEV_AV_OUT_EMPTY
                | signals::USBDEV_AV_SETUP_EMPTY,
            Instant::MAX,
        )?;

        while let Some(event) = usb.poll() {
            let mut action = match usbtpm.handle_event(event) {
                Ok(a) => a,
                Err(e) => ep0.handle_event(e, &descriptors).unwrap_or(UsbAction::None),
            };
            action.run(&mut usb);
        }

        // Initiate any pending transmissions
        usbtpm.poll_transmit(&mut usb);
    }
}

fn usb_setup_pinmux() {
    use top_earlgrey::PinmuxPeripheralIn;
    let mut pinmux = unsafe { pinmux::PinmuxAon::new() };

    pinmux
        .regs_mut()
        .mio_periph_insel()
        .at(PinmuxPeripheralIn::UsbdevSense as usize)
        .modify(|_| (top_earlgrey::PinmuxInsel::ConstantOne as u32).into());
}

#[entry]
fn entry() -> ! {
    pw_log::info!("TPM Bridge: Starting...");
    usb_setup_pinmux();
    
    let ret = handle_usb();
    if let Err(e) = ret {
        pw_log::error!("TPM Bridge: USB handler failed: {}", e as u32);
    }

    let _ = syscall::debug_shutdown(ret);
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    pw_log::error!("TPM Bridge: PANIC!");
    let _ = syscall::debug_shutdown(Err(pw_status::Error::Unknown));
    loop {}
}
