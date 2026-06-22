// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

use crate::pinout::Pinout;
use earlgrey_gpio::{EarlGreyGpio, GpioMask};
use earlgrey_pinmux::{Pad, PadConfig, Pull};
use earlgrey_sysmgr_client::{ResetInfo, SysmgrClient};
use openprot_hal_blocking::gpio_port::{
    EdgeSensitivity, GpioInterrupt, GpioPort, InterruptOperation, PinMask,
};
use userspace::time::{sleep_until, Clock, Duration, Instant, SystemClock};
use util_error::ErrorCode;
use util_ipc::IpcChannel;
use zfmt::Zfmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    ColdBoot,
    LatchReset,
    Measure,
    ReleaseReset,
    Running,
}

#[derive(Zfmt, Clone)]
#[zfmt(format = "Platform State: {state}")]
pub struct StateTransition {
    pub state: &'static str,
}

#[derive(Zfmt, Clone)]
#[zfmt(format = "USB Presence: {present}")]
pub struct UsbPresence {
    pub present: bool,
}

#[derive(Zfmt, Clone)]
#[zfmt(format = "Reset Monitor: {monitor}")]
pub struct ResetMonitor {
    pub monitor: u32,
}

pub struct PlatformServer<IPC: IpcChannel> {
    gpio: EarlGreyGpio,
    sysmgr: SysmgrClient<IPC>,
    state: State,
    next_deadline: Instant,
    exit_deadline: Instant,
}

impl<IPC: IpcChannel> PlatformServer<IPC> {
    pub fn new(ipc: IPC) -> Result<Self, ErrorCode> {
        let gpio = unsafe { EarlGreyGpio::new() };
        let sysmgr = SysmgrClient::new(ipc);
        Ok(Self {
            gpio,
            sysmgr,
            state: State::ColdBoot,
            next_deadline: Instant::MAX,
            exit_deadline: Instant::MAX,
        })
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn next_deadline(&self) -> Instant {
        self.next_deadline.min(self.exit_deadline)
    }

    pub fn set_exit_deadline(&mut self, deadline: Instant) {
        self.exit_deadline = deadline;
    }

    pub fn should_exit(&self) -> bool {
        SystemClock::now() >= self.exit_deadline
    }

    pub fn start(&mut self) -> Result<(), ErrorCode> {
        // 1. Initialize pinmux and GPIO drivers.
        Pinout::configure(&mut self.gpio.pinmux)?;

        // 2. Read SW_STRAPs (always).
        let straps = self.read_straps()?;
        util_zfmt::debug!("SW_STRAPs read: {straps:02x}", straps = straps);

        // 3. Send the strap value to sysmgr (always).
        self.sysmgr.set_software_straps(straps)?;

        // 4. Get BootInfo from sysmgr (always).
        let boot_info = self.sysmgr.get_boot_info()?;

        // 5. Configure interrupts on RST_MON0_N, RST_MON1_N, and USB_PRESENCE_N (always).
        self.setup_interrupts()?;

        // 6. Check reset reason.
        let is_low_power = (boot_info.reset.reason & ResetInfo::REASON_LOW_POWER_EXIT) != 0;

        if !is_low_power {
            // Configure SPI GPIOs.
            self.configure_spi_gpios()?;
            self.transition_to_latch_reset()?;
        } else {
            self.transition_to_running();
        }

        Ok(())
    }

    fn read_straps(&mut self) -> Result<u32, ErrorCode> {
        let mut strap_value = 0u32;
        let pins = [Pinout::SW_STRAP0, Pinout::SW_STRAP1, Pinout::SW_STRAP2];
        let pads = [Pad::IOC0, Pad::IOC1, Pad::IOC2];

        for (i, (&pin, &pad)) in pins.iter().zip(pads.iter()).enumerate() {
            let pin_mask = GpioMask::from(pin);

            // 1. Configure no pull
            self.gpio.pinmux.configure_pad(
                pad,
                &PadConfig {
                    pull: Pull::None,
                    ..Default::default()
                },
            )?;

            // 2. Delay 50us
            sleep_until(SystemClock::now() + Duration::from_micros(50))
                .map_err(ErrorCode::kernel_error)?;

            // 3. Read val1
            let val1 = if self
                .gpio
                .read_input()
                .map_err(ErrorCode::from)?
                .contains(pin_mask)
            {
                1
            } else {
                0
            };

            // 4. Configure pull opposite to val1
            let pull = if val1 == 0 { Pull::Up } else { Pull::Down };
            self.gpio.pinmux.configure_pad(
                pad,
                &PadConfig {
                    pull,
                    ..Default::default()
                },
            )?;

            // 5. Delay 50us
            sleep_until(SystemClock::now() + Duration::from_micros(50))
                .map_err(ErrorCode::kernel_error)?;

            // 6. Read val2
            let val2 = if self
                .gpio
                .read_input()
                .map_err(ErrorCode::from)?
                .contains(pin_mask)
            {
                1
            } else {
                0
            };

            // 7. Result = (val1 << 1) | val2
            let pin_res = (val1 << 1) | val2;
            strap_value |= pin_res << (i * 2);
        }

        Ok(strap_value)
    }

    fn setup_interrupts(&mut self) -> Result<(), ErrorCode> {
        // USB_PRESENCE_N (Pin 16) -> AnyEdge
        let usb_pres = GpioMask::from(Pinout::USB_PRESENCE_N);
        self.gpio
            .irq_configure(usb_pres, EdgeSensitivity::BothEdges)
            .map_err(ErrorCode::from)?;
        self.gpio
            .irq_control(usb_pres, InterruptOperation::Enable)
            .map_err(ErrorCode::from)?;

        // RST_MON0_N (Pin 17) -> FallingEdge
        let rst_mon0 = GpioMask::from(Pinout::RST_MON0_N);
        self.gpio
            .irq_configure(rst_mon0, EdgeSensitivity::FallingEdge)
            .map_err(ErrorCode::from)?;
        self.gpio
            .irq_control(rst_mon0, InterruptOperation::Enable)
            .map_err(ErrorCode::from)?;

        // RST_MON1_N (Pin 18) -> FallingEdge
        let rst_mon1 = GpioMask::from(Pinout::RST_MON1_N);
        self.gpio
            .irq_configure(rst_mon1, EdgeSensitivity::FallingEdge)
            .map_err(ErrorCode::from)?;
        self.gpio
            .irq_control(rst_mon1, InterruptOperation::Enable)
            .map_err(ErrorCode::from)?;

        Ok(())
    }

    fn configure_spi_gpios(&mut self) -> Result<(), ErrorCode> {
        let low_pins =
            GpioMask::from(Pinout::SPI_MUX_CTRL).union(GpioMask::from(Pinout::SPI_MUX_EN_N));
        let high_pins = GpioMask::from(Pinout::SPI_RESET_N)
            .union(GpioMask::from(Pinout::SPI_HOST0_WP_N))
            .union(GpioMask::from(Pinout::SPI_HOST1_WP_N));

        self.gpio
            .set_reset(high_pins, low_pins)
            .map_err(ErrorCode::from)?;
        Ok(())
    }

    fn transition_to_latch_reset(&mut self) -> Result<(), ErrorCode> {
        self.state = State::LatchReset;
        util_zfmt::info!(StateTransition {
            state: "LatchReset"
        });
        // Drive RST_CTRL0_N Low
        self.gpio
            .set_reset(GpioMask::empty(), GpioMask::from(Pinout::RST_CTRL0_N))
            .map_err(ErrorCode::from)?;

        self.transition_to_measure();
        Ok(())
    }

    fn transition_to_measure(&mut self) {
        self.state = State::Measure;
        util_zfmt::info!(StateTransition { state: "Measure" });
        // Set 1s deadline
        self.next_deadline = SystemClock::now() + Duration::from_secs(1);
    }

    fn transition_to_release_reset(&mut self) -> Result<(), ErrorCode> {
        self.state = State::ReleaseReset;
        util_zfmt::info!(StateTransition {
            state: "ReleaseReset"
        });
        // Drive RST_CTRL0_N High
        self.gpio
            .set_reset(GpioMask::from(Pinout::RST_CTRL0_N), GpioMask::empty())
            .map_err(ErrorCode::from)?;

        self.transition_to_running();
        Ok(())
    }

    fn transition_to_running(&mut self) {
        self.state = State::Running;
        util_zfmt::info!(StateTransition { state: "Running" });
        self.next_deadline = Instant::MAX;
    }

    pub fn handle_timeout(&mut self) -> Result<(), ErrorCode> {
        match self.state {
            State::Measure => {
                self.transition_to_release_reset()?;
            }
            _ => {
                self.next_deadline = Instant::MAX;
            }
        }
        Ok(())
    }

    pub fn handle_usb_presence_interrupt(&mut self) -> Result<(), ErrorCode> {
        let pin_mask = GpioMask::from(Pinout::USB_PRESENCE_N);
        self.gpio
            .irq_control(pin_mask, InterruptOperation::Clear)
            .map_err(ErrorCode::from)?;

        let is_high = self
            .gpio
            .read_input()
            .map_err(ErrorCode::from)?
            .contains(pin_mask);
        let usb_mux = GpioMask::from(Pinout::USB_MUX_CTRL);
        if is_high {
            self.gpio
                .set_reset(usb_mux, GpioMask::empty())
                .map_err(ErrorCode::from)?;
            util_zfmt::info!(UsbPresence { present: false });
        } else {
            self.gpio
                .set_reset(GpioMask::empty(), usb_mux)
                .map_err(ErrorCode::from)?;
            util_zfmt::info!(UsbPresence { present: true });
        }
        Ok(())
    }

    pub fn handle_rst_mon_interrupt(&mut self, index: usize) -> Result<(), ErrorCode> {
        let pin = if index == 0 {
            Pinout::RST_MON0_N
        } else {
            Pinout::RST_MON1_N
        };
        let pin_mask = GpioMask::from(pin);

        self.gpio
            .irq_control(pin_mask, InterruptOperation::Clear)
            .map_err(ErrorCode::from)?;

        util_zfmt::info!(ResetMonitor {
            monitor: index as u32
        });

        self.transition_to_latch_reset()?;
        Ok(())
    }
}
