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

## USB Class Builder Convention

To maintain zero-cost `const` initialization while supporting modular class
implementations, all USB classes should provide a companion `Builder` struct.

### Builder Requirements

1.  **Fragment Calculation**: Provide `const fn` methods to calculate child
    descriptor arrays (e.g., `[EndpointDescriptor; N]`) and hardware config
    (`[EpIn; N]`, `[EpOut; N]`).
2.  **Interface Encapsulation**: Provide `const fn` methods to construct fully
    populated `InterfaceDescriptor` structs. These methods should take references
    to `static` fragments and `StringHandle`s, hiding class-specific magic
    numbers (class codes, protocol types) from the application.
3.  **Class Initialization**: The class's `new()` method should accept its
    corresponding `Builder` as a configuration object.

### Example Usage (Application)

```rust
const MY_BUILDER: MyClassBuilder = MyClassBuilder::new(IF_NUM, EP_NUM);

// Static storage for fragments (required for 'static lifetimes)
static MY_FUNC_DESCS: [FunctionalDescriptor; 1] = MY_BUILDER.functional_descriptors();
static MY_ENDPOINTS: [EndpointDescriptor; 1] = MY_BUILDER.endpoints();

// Fully encapsulated interface construction
const MY_INTERFACE: InterfaceDescriptor = MY_BUILDER.interface(NAME_HANDLE, &MY_FUNC_DESCS, &MY_ENDPOINTS);

// Hardware config
const EPS: ([EpIn; 1], [EpOut; 0]) = MY_BUILDER.eps();
```

## Files

- `BUILD.bazel`: Bazel rules for building and testing this library.
- `descriptor.rs`: Structures and serialization logic for USB descriptors.
- `driver.rs`: Traits for hardware-specific USB driver implementations.
- `lib.rs`: Common USB definitions and the `SetupPacket` structure.
