# About `protocol/usb/stack`

This directory contains a generic, lightweight USB device stack designed for
embedded systems. It provides the core logic for handling standard USB requests
and managing device state in an OS-agnostic manner.

## Architecture

The stack is designed around a poll-based event loop and is decoupled from
specific hardware through the `UsbDriver` HAL defined in `//hal/blocking/usb`.

### Key Components

- **`SimpleEp0`**: Implements the standard control requests on Endpoint 0. It
  handles common requests like `GET_DESCRIPTOR`, `SET_ADDRESS`, and
  `SET_CONFIGURATION`. It uses a `DescriptorSource` to retrieve the device-specific
  descriptors.
- **`DescriptorSource`**: A trait that must be implemented by the application to
  provide the USB descriptors (Device, Configuration, String, etc.) to the stack.
- **`UsbAction`**: An enum that captures the side-effects of a USB event.
  Actions include sending data (`TransferIn`), stalling endpoints, or changing
  the device address. These actions are intended to be executed by the driver.
- **`Transfer<const N: usize>`**: A utility for accumulating multi-packet USB
  transfers. it handles short-packet and ZLP termination logic.

## Usage

Applications typically instantiate a `SimpleEp0` and a `UsbDriver`. In a loop
(or interrupt handler), the driver is polled for events. These events are
passed to `SimpleEp0::handle_event`, which returns a `UsbAction`. This action
is then executed using `UsbAction::run(driver)`.

```rust
let mut ep0 = SimpleEp0::new();
let mut driver = MyUsbDriver::new();
let descriptors = MyDescriptors::new();

loop {
    if let Some(event) = driver.poll() {
        let mut action = ep0.handle_event(event, &descriptors);
        action.run(&mut driver);
    }
}
```

## Files

- `BUILD.bazel`: Bazel rules for building and testing this library.
- `lib.rs`: Implementation of the USB stack logic.
