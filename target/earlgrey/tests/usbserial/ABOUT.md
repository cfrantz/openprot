# About `target/earlgrey/tests/usbserial`

This directory contains a complete integration test for the USB stack and the
Earlgrey USB driver, implementing a CDC-ACM (Serial) device.

## Overview

The test program demonstrates a functional USB device on the Pigweed Maize
kernel. It implements a simple echo service: any data received over the
virtual serial port is echoed back to the host.

## Integration with Pigweed Maize

The test utilizes the Pigweed Maize kernel's static configuration and
component-based architecture.

### Key Components

- **`system.json5`**: Defines the static system configuration, including
  component instances, memory allocations, and capabilities.
- **`target.rs`**: The program entry point. It initializes the kernel and
  starts the defined components.
- **`test_usb.rs`**: The main test logic. It runs as a userspace component,
  interacting with the USB hardware through the `Usbdev` driver and
  the USB stack.

## Testing Strategy

The test verifies:
1. **USB Enumeration**: The device correctly identifies itself as a CDC-ACM device.
2. **Control Transfers**: The USB stack correctly handles standard and
   class-specific control requests.
3. **Data Transfers**: Bulk IN and OUT transfers are functional, and data
   integrity is maintained during echoing.

## Files

- `BUILD.bazel`: Build and test rules for the USB serial test.
- `system.json5`: Static system configuration for the test environment.
- `target.rs`: Bootstrapping logic for the Maize kernel.
- `test_usb.rs`: The implementation of the CDC-ACM echo test.
