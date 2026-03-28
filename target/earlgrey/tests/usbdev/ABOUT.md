# About `target/earlgrey/tests/usbdev`

This directory contains a minimal enumeration test for the Earlgrey USB driver.

## Overview

The test program provides a baseline verification of the `Usbdev` driver. Its
primary goal is to successfully enumerate the device on the USB bus,
demonstrating that the driver can correctly handle basic hardware
initialization and Endpoint 0 communication.

## Architecture

This test uses the Pigweed Maize kernel's static system configuration to
isolate the USB driver and test logic.

### Key Components

- **`system.json5`**: Static configuration defining the USB component's
  capabilities and memory limits.
- **`target.rs`**: Entry point for kernel initialization and test execution.
- **`test_usb.rs`**: Core test implementation, focusing on driver
  configuration and standard USB request handling.

## Testing Strategy

The test verifies:
1. **Hardware Initialization**: The `Usbdev` driver correctly configures
   the peripheral hardware.
2. **Standard Requests**: The driver and stack correctly respond to
   `GET_DESCRIPTOR` and `SET_ADDRESS` requests.
3. **Interrupt Handling**: USB events correctly trigger driver responses.

## Files

- `BUILD.bazel`: Build and test rules for the USB enumeration test.
- `system.json5`: Static system configuration.
- `target.rs`: Kernel bootstrapping logic.
- `test_usb.rs`: Minimal USB enumeration test logic.
