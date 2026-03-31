# USB Protocol Stack

The OpenPRoT USB protocol stack is a lightweight, `no_std` implementation designed for root-of-trust firmware. It prioritizes static configuration and efficient event handling.

## Key Features

- **Static Descriptor Initialization**: Most USB descriptors can be defined as `const` data, reducing runtime overhead and memory usage.
- **Modular Interface**: The `UsbClass` trait allows for clean separation between different USB functions (e.g., CDC-ACM, HID, DFU).
- **Simplified Event Loop**: A simple `poll()`-based architecture avoids complex callback structures, making it easier to integrate with various RTOSs or bare-metal loops.
- **Multifunction Support**: Multiple USB classes can be easily combined into a single device by chaining their event handlers.

## Initialization Guide

### 1. Define Descriptors

Use the `hal_usb` types to define your device and configuration descriptors. These are typically `const`.

```rust
const DEVICE_DESC: DeviceDescriptor = DeviceDescriptor {
    device_class: hal_usb::DeviceClass::SPECIFIED_BY_INTERFACE,
    // ... other fields
};

const CONFIG_DESC: ConfigDescriptor = ConfigDescriptor {
    configuration_value: 1,
    interfaces: &[
        // Define your interfaces here
    ],
    // ...
};
```

### 2. Implement `DescriptorSource`

The `DescriptorSource` trait provides the stack with the raw bytes for descriptors and handles string descriptor lookups.

```rust
struct MyDescriptors;

impl DescriptorSource for MyDescriptors {
    const DEVICE_DESC_BYTES: &'static Aligned<A4, [u8]> = &Aligned(DEVICE_DESC.serialize());
    const CONFIG_DESC_BYTES: &'static Aligned<A4, [u8]> =
        &Aligned(CONFIG_DESC.serialize::<{ CONFIG_DESC.total_size() }>());
    const STRING_DESC_0_BYTES: &'static Aligned<A4, [u8]> =
        &Aligned(STRING_DESC_0.serialize::<{ STRING_DESC_0.total_size() }>());
    const DEVICE_STATUS: Aligned<A4, [u8; 2]> = Aligned([1u8, 0]);

    fn get_string(&self, handle: StringHandle, lang: u16) -> Option<StringDescriptorRef<'_>> {
        match handle {
            // Map handles to string descriptors
            _ => None,
        }
    }
}
```

### 3. Initialize the Driver and Classes

Setup the hardware driver, the USB stack configuration, and any specific class implementations. For multifunction devices, you must combine the endpoint lists from each class builder into a single `UsbConfig`.

```rust
// Get endpoint definitions from class builders
const CDC_EPS: ([EpIn; 2], [EpOut; 1]) = CDC_BUILDER.eps();
// IF MULTIFUNCTION: const HID_EPS: ([EpIn; 1], [EpOut; 1]) = HID_BUILDER.eps();

// Combine into single IN and OUT lists
const USB_CONFIG: UsbConfig = UsbConfig::new(
    &[
        CDC_EPS.0[0],
        CDC_EPS.0[1],
        // IF MULTIFUNCTION: HID_EPS.0[0],
    ],
    &[
        CDC_EPS.1[0],
        // IF MULTIFUNCTION: HID_EPS.1[0],
    ],
);

let mut usb = usb_driver::Usb::new(usb_hardware, USB_CONFIG);
let mut ep0 = usb_stack::SimpleEp0::new();
let mut cdc_acm = CdcAcm::<1024, 1024>::new(CDC_BUILDER);
// IF MULTIFUNCTION: let mut hid = Hid::new(HID_BUILDER);
```

### 4. The Poll Loop

The main loop polls for events and dispatches them to the classes. The `handle_event` method returns `Ok(UsbAction)` if the event was handled, or `Err(event)` if it should be passed to the next handler. Chaining these with `or_else` provides a clean way to handle multifunction devices.

```rust
while let Some(event) = usb.poll() {
    // Dispatch event to classes in order using or_else for clean chaining
    let action = cdc_acm.handle_event(event)
        // IF MULTIFUNCTION: Add additional classes here
        // .or_else(|event| hid.handle_event(event))
        // .or_else(|event| msc.handle_event(event))
        .unwrap_or_else(|event| {
            // Default to Endpoint 0 handling
            ep0.handle_event(event, &descriptors).unwrap_or(UsbAction::None)
        });
    
    // Execute the resulting action (e.g., send an IN packet, set address, etc.)
    action.run(&mut usb);
}
```

## Example: CDC-ACM Initialization

This example shows a minimal setup for a USB Serial (CDC-ACM) device.

```rust
use hal_usb::{ConfigDescriptor, DeviceDescriptor};
use usb_stack::{DescriptorSource, UsbAction};
use protocol_usb_cdc_acm::{CdcAcm, CdcAcmBuilder};

// 1. Define handles for strings
const USB_VENDOR_HANDLE: hal_usb::StringHandle = hal_usb::StringHandle(1);
const USB_PRODUCT_HANDLE: hal_usb::StringHandle = hal_usb::StringHandle(2);

// 2. Configure the CDC-ACM Builder
const CDC_BUILDER: CdcAcmBuilder = CdcAcmBuilder::new(
    0, // comm_if: Communication Interface index
    1, // data_if: Data Interface index
    1, // comm_ep: Communication IN endpoint
    2, // data_out_ep: Data OUT endpoint
    3, // data_in_ep: Data IN endpoint
);

// 3. Define the Device Descriptor
const DEVICE_DESC: DeviceDescriptor = DeviceDescriptor {
    device_class: hal_usb::DeviceClass::SPECIFIED_BY_INTERFACE,
    vendor_id: 0x18d1,
    product_id: 0x023b,
    manufacturer: USB_VENDOR_HANDLE,
    product: USB_PRODUCT_HANDLE,
    // ...
    ..DeviceDescriptor::default()
};

// 4. Define the Configuration Descriptor
const CONFIG_DESC: ConfigDescriptor = ConfigDescriptor {
    configuration_value: 1,
    interfaces: &[
        CDC_BUILDER.comm_interface(
            hal_usb::StringHandle(0),
            &CDC_BUILDER.comm_func_descs(),
            &CDC_BUILDER.comm_endpoints(),
        ),
        CDC_BUILDER.data_interface(hal_usb::StringHandle(0), &CDC_BUILDER.data_endpoints()),
        // IF MULTIFUNCTION: Add additional interfaces from other builders here
    ],
    ..ConfigDescriptor::default()
};

// ... Implement DescriptorSource and run the poll loop as shown above
```
