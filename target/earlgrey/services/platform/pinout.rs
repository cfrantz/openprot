use earlgrey_gpio::GpioPin;
use earlgrey_pinmux::{EarlGreyPinmux, Pad, PadConfig, Pull};
use top_earlgrey::{PinmuxOutsel as Outsel, PinmuxPeripheralIn as PeriphIn};
use util_error::ErrorCode;

pub enum Config {
    Input {
        periph: PeriphIn,
        pad: Pad,
        pad_config: PadConfig,
    },
    Output {
        periph: Outsel,
        pad: Pad,
        pad_config: PadConfig,
    },
    Io {
        periph: PeriphIn,
        pad: Pad,
        pad_config: PadConfig,
    },
}

impl Config {
    pub fn apply(&self, pinmux: &mut EarlGreyPinmux) -> Result<(), ErrorCode> {
        match self {
            Self::Input {
                periph,
                pad,
                pad_config,
            } => {
                pinmux.configure_pad(*pad, pad_config)?;
                pinmux.connect_input(*periph, *pad)?;
            }
            Self::Output {
                periph,
                pad,
                pad_config,
            } => {
                pinmux.configure_pad(*pad, pad_config)?;
                pinmux.connect_output(*pad, *periph)?;
            }
            Self::Io {
                periph,
                pad,
                pad_config,
            } => {
                pinmux.configure_pad(*pad, pad_config)?;
                pinmux.connect_input(*periph, *pad)?;
                // For the range of peripherals that are valid as input and output,
                // (Gpio0 to SpiHost1Sd3), the offset between PeriphIn and Outsel
                // is 3.
                // TODO: should we check this and return an error?
                let outsel = Outsel::try_from(*periph as u32 + 3).unwrap();
                pinmux.connect_output(*pad, outsel)?;
            }
        }
        Ok(())
    }
}

pub struct PinoutConfig {
    pub name: &'static str,
    pub padname: &'static str,
    pub pin: Option<GpioPin>,
    pub config: Config,
}

impl PinoutConfig {
    const fn gpio_in(
        name: &'static str,
        padname: &'static str,
        pin: GpioPin,
        pad: Pad,
        pad_config: PadConfig,
    ) -> PinoutConfig {
        PinoutConfig {
            name,
            padname,
            pin: Some(pin),
            config: Config::Input {
                periph: pin.as_periph(),
                pad,
                pad_config,
            },
        }
    }

    const fn gpio_out(
        name: &'static str,
        padname: &'static str,
        pin: GpioPin,
        pad: Pad,
        pad_config: PadConfig,
    ) -> PinoutConfig {
        PinoutConfig {
            name,
            padname,
            pin: Some(pin),
            config: Config::Output {
                periph: pin.as_outsel(),
                pad,
                pad_config,
            },
        }
    }

    const fn func_in(
        name: &'static str,
        padname: &'static str,
        periph: PeriphIn,
        pad: Pad,
        pad_config: PadConfig,
    ) -> PinoutConfig {
        PinoutConfig {
            name,
            padname,
            pin: None,
            config: Config::Input {
                periph,
                pad,
                pad_config,
            },
        }
    }

    const fn func_out(
        name: &'static str,
        padname: &'static str,
        periph: Outsel,
        pad: Pad,
        pad_config: PadConfig,
    ) -> PinoutConfig {
        PinoutConfig {
            name,
            padname,
            pin: None,
            config: Config::Output {
                periph,
                pad,
                pad_config,
            },
        }
    }

    const fn func_io(
        name: &'static str,
        padname: &'static str,
        periph: PeriphIn,
        pad: Pad,
        pad_config: PadConfig,
    ) -> PinoutConfig {
        PinoutConfig {
            name,
            padname,
            pin: None,
            config: Config::Io {
                periph,
                pad,
                pad_config,
            },
        }
    }
}

pub const IN_PULL_NONE: PadConfig = PadConfig {
    pull: Pull::None,
    open_drain: false,
    invert: false,
};
pub const IN_PULL_UP: PadConfig = PadConfig {
    pull: Pull::Up,
    open_drain: false,
    invert: false,
};
pub const IN_PULL_DOWN: PadConfig = PadConfig {
    pull: Pull::Down,
    open_drain: false,
    invert: false,
};
pub const OUT_PUSH_PULL: PadConfig = PadConfig {
    pull: Pull::None,
    open_drain: false,
    invert: false,
};
pub const OUT_PULL_UP: PadConfig = PadConfig {
    pull: Pull::Up,
    open_drain: true,
    invert: false,
};
pub const OUT_PULL_DOWN: PadConfig = PadConfig {
    pull: Pull::Down,
    open_drain: true,
    invert: false,
};

type PC = PinoutConfig;

pub struct Pinout;
impl Pinout {
    // Outputs use Gpio [0..15].
    pub const RST_CTRL0_N: GpioPin = GpioPin::Pin0;
    pub const RST_CTRL1_N: GpioPin = GpioPin::Pin1;
    pub const SPI_RESET_N: GpioPin = GpioPin::Pin2;
    pub const SPI_MUX_EN_N: GpioPin = GpioPin::Pin3;
    pub const SPI_MUX_CTRL: GpioPin = GpioPin::Pin4;
    pub const SPI_HOST0_WP_N: GpioPin = GpioPin::Pin5;
    pub const SPI_HOST1_WP_N: GpioPin = GpioPin::Pin6;
    pub const USB_MUX_CTRL: GpioPin = GpioPin::Pin7;
    pub const EXT_DEBUG_N: GpioPin = GpioPin::Pin8;

    // Inputs use Gpio [16..31].
    pub const USB_PRESENCE_N: GpioPin = GpioPin::Pin16;
    pub const RST_MON0_N: GpioPin = GpioPin::Pin17;
    pub const RST_MON1_N: GpioPin = GpioPin::Pin18;
    pub const SW_STRAP0: GpioPin = GpioPin::Pin22;
    pub const SW_STRAP1: GpioPin = GpioPin::Pin23;
    pub const SW_STRAP2: GpioPin = GpioPin::Pin24;

    #[rustfmt::skip]
    pub const PINOUT_DUAL_SBS: [PinoutConfig; 23] = [
        PC::func_in( "UART0_RX",       "IOC3",  PeriphIn::Uart0Rx,         Pad::IOC3, IN_PULL_UP),
        PC::func_out("UART0_TX",       "IOC4",  Outsel::Uart0Tx,           Pad::IOC4, OUT_PULL_UP),
        PC::func_in( "USBDEV_SENSE",   "none",  PeriphIn::UsbdevSense,     Pad::ConstantOne, IN_PULL_NONE),
        PC::gpio_in( "USB_PRESENCE_N", "IOR11", Self::USB_PRESENCE_N,      Pad::IOR11, IN_PULL_UP),
        PC::gpio_out("USB_MUX_CTRL",   "IOC6",  Self::USB_MUX_CTRL,        Pad::IOC6, OUT_PUSH_PULL),
        PC::gpio_out("RST_CTRL0_N",    "IOA0",  Self::RST_CTRL0_N,         Pad::IOA0, OUT_PUSH_PULL),
        PC::gpio_out("RST_CTRL1_N",    "IOA1",  Self::RST_CTRL1_N,         Pad::IOA1, OUT_PUSH_PULL),
        PC::gpio_in( "RST_MON0_N",     "IOA2",  Self::RST_MON0_N,          Pad::IOA2, IN_PULL_NONE),
        PC::gpio_in( "RST_MON1_N",     "IOA5",  Self::RST_MON1_N,          Pad::IOA5, IN_PULL_NONE),
        PC::gpio_out("SPI_RESET_N",    "IOA7",  Self::SPI_RESET_N,         Pad::IOA7, OUT_PULL_UP),
        PC::func_in( "SPI_DEV_CS1_L",  "IOA4",  PeriphIn::SpiDeviceTpmCsb, Pad::IOA4, IN_PULL_UP),
        PC::gpio_out("SPI_MUX_EN_N",   "IOB7",  Self::SPI_MUX_EN_N,        Pad::IOB7, IN_PULL_UP),
        PC::gpio_out("SPI_MUX_CTRL",   "IOB8",  Self::SPI_MUX_CTRL,        Pad::IOB8, IN_PULL_UP),
        PC::gpio_out("SPI_HOST0_WP_N", "IOA3",  Self::SPI_HOST0_WP_N,      Pad::IOA3, OUT_PULL_UP),
        PC::gpio_out("SPI_HOST1_WP_N", "IOA6",  Self::SPI_HOST1_WP_N,      Pad::IOA6, OUT_PULL_UP),
        PC::gpio_out("EXT_DEBUG_N",    "IOC9",  Self::EXT_DEBUG_N,         Pad::IOC6, OUT_PULL_UP),
        PC::func_out("SPI_HOST1_CLK",  "IOB0",  Outsel::SpiHost1Sck,       Pad::IOB0, OUT_PULL_UP),
        PC::func_out("SPI_HOST1_CS_L", "IOB3",  Outsel::SpiHost1Csb,       Pad::IOB3, OUT_PULL_UP),
        PC::func_io( "SPI_HOST1_D0",   "IOB1",  PeriphIn::SpiHost1Sd0,     Pad::IOB1, IN_PULL_UP),
        PC::func_io( "SPI_HOST1_D1",   "IOB2",  PeriphIn::SpiHost1Sd1,     Pad::IOB2, IN_PULL_UP),
        PC::gpio_in( "SW_STRAP0",      "IOC0",  Self::SW_STRAP0,           Pad::IOC0, IN_PULL_NONE),
        PC::gpio_in( "SW_STRAP1",      "IOC1",  Self::SW_STRAP1,           Pad::IOC1, IN_PULL_NONE),
        PC::gpio_in( "SW_STRAP2",      "IOC2",  Self::SW_STRAP2,           Pad::IOC2, IN_PULL_NONE),
    ];

    pub fn configure(pinmux: &mut EarlGreyPinmux) -> Result<(), ErrorCode> {
        for pin in Self::PINOUT_DUAL_SBS.iter() {
            pin.config.apply(pinmux)?;
        }
        Ok(())
    }
}
