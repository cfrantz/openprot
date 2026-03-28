# About `hal/blocking/usb`

This directory defines the Hardware Abstraction Layer (HAL) for USB peripheral
devices. It provides the traits and structures needed to implement hardware-specific
USB drivers and create hardware-agnostic USB device stacks.

## Key Traits

- **`UsbDriver`**: The primary trait implemented by hardware drivers for USB peripheral
  controllers. It defines methods for data transfers (`transfer_in`), endpoint
  stalling, setting device addresses, and polling for USB events.
- **`UsbPacket`**: A trait representing a received USB packet. It provides methods
  to read packet metadata (endpoint, length) and copy packet data into system memory.

## Key Data Structures

- **`SetupPacket`**: Represents a standard USB SETUP packet, providing structured
  access to request types, request codes, values, indices, and lengths.
- **`UsbEvent`**: An enum representing events from the USB controller, such as
  receiving a SETUP or data packet, connection state changes (VBUS), or USB resets.
- **Descriptor Structs**: `DeviceDescriptor`, `ConfigDescriptor`,
  `InterfaceDescriptor`, and `EndpointDescriptor` provide a structured way to
  define and serialize USB descriptors.

## Key Utilities

- **`StringDescriptor`**: Simplifies the creation of UTF-16 string descriptors
  from ASCII strings.
- **`StringDescriptorWritter`**: A utility for dynamically building string
  descriptors.
- **`FunctionalDescriptor`**: Support for serialization of class-specific
  functional descriptors (e.g., DFU functional descriptors).

## Files

- `BUILD.bazel`: Bazel rules for building and testing this library.
- `descriptor.rs`: Structures and serialization logic for USB descriptors.
- `driver.rs`: Traits for hardware-specific USB driver implementations.
- `lib.rs`: Common USB definitions and the `SetupPacket` structure.
