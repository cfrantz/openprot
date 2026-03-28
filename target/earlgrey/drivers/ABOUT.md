# About `target/earlgrey/drivers`

This directory provides hardware-specific driver implementations for the
OpenTitan Earlgrey system-on-chip.

## Architecture

These drivers are designed to be OS-agnostic and focus solely on hardware
control. They implement the HAL traits defined in `//hal/blocking/...`.

### Key Components

- **`Usbdev`**: An implementation of the `UsbDriver` trait for the Earlgrey
  USB peripheral controller. It manages hardware buffers, handles interrupts,
  and provides a poll-based interface for the USB stack.
- **Hardware Integration**: The drivers are built using Pigweed's toolchain
  and libraries, but are designed to be portable across different RTOSs or
  bare-metal environments.

## Files

- `BUILD.bazel`: Bazel rules for building the Earlgrey-specific drivers.
- `usbdev.rs`: The implementation of the Earlgrey USB peripheral driver.
