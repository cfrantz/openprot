# TCTI usb-test-protocol

This document describes a custom USB protcol for communicating with TPMs.
Although it is not recommended to connect a TPM via USB, this protocol is useful
for development and debug scenarios.

## High-level details

This protocol is _very_ roughly inspired by the USB bulk-only storage protocol.
The main point of inspiration is the way bulk-only manages request/response data
flow:

1. The host sends a request header that includes a command code and a length on
   the OUT endpoint.
2. The host sends the request payload on the OUT endpoint.
3. The device processes the request.
4. The device responds with a response header that includes a status code and a
   length on the IN endpoint.
5. The device responds with the response payload on the IN endpoint.

The device can issue a STALL on the OUT endpoint after the request header if it
rejects the command code.  The host clears the stall using the standard USB
`CLEAR_FEATURE` request.

## USB Interfaces

This protocol identifies itself via the USB interface descriptor.  The
descriptor contains one Bulk OUT and one Bulk IN endpoint.  This protocol is a
non-standard vendor-defined protocol.  It uses the vendor-defined interface
class of 0xFF.  The interface subclass is 0xCF.  The interface protocol is 0x66.

There are no requirements on the endpoint sizes apart from standard USB
requirements.

## Protocol Structure

The request and response headers share a common structure and use different
identifier words to differentiate.  The identifier words use the `FourCC` scheme
so that they're easily identifiable by humans reading hexdumps.

All multi-byte fields in the `UsbTpmHeader` are represented in little-endian
format.  TPM command and response buffers remain in big-endian format as
specified by the TCG.

The common structure is as follows (in C):
```c
struct UsbTpmHeader {
    uint32_t header;        // Always the FourCC `TPM_` (0x5F4D5054).
                            // This field is used as a synchronization marker.
                            // If synchronization is lost, the receiver should
                            // scan the incoming stream for this value. The
                            // header is guaranteed to be aligned to a USB
                            // packet boundary.
    uint32_t identifier;    // Uses the FourCC `_REQ` (0x5145525F) for requests
                            // and `_RSP` (0x5053525F) for responses.
    union {
        uint32_t command;   // The protocol command code when this is a request.
        uint32_t status;    // The protocol result code when this is a response.
    };
    uint32_t length;        // The length of the following payload transfer.
                            // This is the authoritative source of truth for
                            // the USB transfer size. It is an error if this
                            // length is shorter than the length encoded within
                            // the payload (e.g., in the TPM command header).
};
```

### Command Codes

The following command codes are defined for the `command` field in a request:

| Name | Code | Description | Payload |
| :--- | :--- | :---------- | :------ |
| `MS_SIM_TPM_SEND_COMMAND` | `8`  | Execute a raw TPM command. | Raw TPM command buffer. |
| `TPM_SESSION_END`         | `20` | End the session. | None |

### Status Codes

The following status codes are defined for the `status` field in a response:

| Name | Code | Description | TCTI RC Mapping |
| :--- | :--- | :---------- | :-------------- |
| `STATUS_SUCCESS`         | `0`  | Command executed successfully (similar to ABSL `kOk`). | `TSS2_RC_SUCCESS` |
| `STATUS_ERR_UNKNOWN_CMD` | `12` | The command code is not recognized (similar to ABSL `kUnimplemented`). | `TSS2_TCTI_RC_NOT_SUPPORTED` |
| `STATUS_ERR_INTERNAL`    | `13` | An internal error occurred in the device (similar to ABSL `kInternal`). | `TSS2_TCTI_RC_GENERAL_FAILURE` |
| `STATUS_ERR_BUSY`        | `14` | The device is currently busy (similar to ABSL `kUnavailable`). | `TSS2_TCTI_RC_TRY_AGAIN` |

As is common practice for USB protocols, the end of a transfer is signalled by a
short packet or zero-length packet in the case where a transfer ends on a packet
sized boundary.

The request is two transfers: the header and the payload.
The response is two transfers: the header and the payload.

## Control Requests

Certain platform-level operations are implemented using USB control transfers
rather than bulk transfers. These requests use the `Vendor` request type and are
directed at the `Interface`.

### Set Locality

The `setLocality` operation is used to inform the device of the desired locality
for subsequent TPM commands.

| Field          | Value                          | Description                             |
| :------------- | :----------------------------- | :-------------------------------------- |
| `bmRequestType`| `0x41` (Vendor \| Interface \| Out) | Vendor-defined request directed at the interface. |
| `bRequest`     | `0x01`                         | The `setLocality` request code.         |
| `wValue`       | `locality`                     | The locality value to set (0-255).      |
| `wIndex`       | `interface_number`             | The USB interface number.               |
| `wLength`      | `0`                            | No payload.                             |

### Cancel

The `cancel` operation is used to signal the device to abort the current TPM
command. It is a one-shot request.

| Field          | Value                          | Description                             |
| :------------- | :----------------------------- | :-------------------------------------- |
| `bmRequestType`| `0x41` (Vendor \| Interface \| Out) | Vendor-defined request directed at the interface. |
| `bRequest`     | `0x02`                         | The `cancel` request code.              |
| `wValue`       | `0`                            | Reserved.                               |
| `wIndex`       | `interface_number`             | The USB interface number.               |
| `wLength`      | `0`                            | No payload.                             |

## TCTI Client Implementation

A TCTI client implementation for this protocol should follow the standard TCTI
interface using synchronous I/O. It is recommended to use `libusb` for USB
communication.

### Configuration

The TCTI configuration string is used to identify the target USB device. It
should support identifying devices by their USB serial number to allow for
multiple devices to be connected simultaneously.

The configuration string format is a key-value string:
`"serial=<serial_number>,timeout=<ms>"`

- `serial`: The USB serial number of the target device.
- `timeout`: The USB transfer timeout in milliseconds. Defaults to `5000`.

If the serial number is not provided and multiple matching devices are found,
the client should connect to the first device it finds, emit a warning
indicating that multiple devices were found, and include the serial number of
the chosen device in the warning.

### udev Rules

Since this protocol uses a custom interface class, the device is typically only
accessible by the `root` user by default on Linux. To allow access for non-root
users, an appropriate `udev` rule should be created.

A common approach is to grant access to members of the `plugdev` group. A sample
rule matching the interface class/subclass/protocol is shown below:

```udev
# TCTI usb-test-protocol device rule
# Matches interface class 0xFF, subclass 0xCF, protocol 0x66
SUBSYSTEM=="usb", ATTR{bInterfaceClass}=="ff", ATTR{bInterfaceSubClass}=="cf", ATTR{bInterfaceProtocol}=="66", GROUP="plugdev", MODE="0660"
```

### Initialization and Lifetime

The implementation should be loosely modeled after `tcti-mssim.c`.

1. **Initialization**:
   - Parse the configuration string to extract the `serial` parameter.
   - Initialize `libusb` using a private `libusb_context` to ensure isolation
     from other library users in the same process.
   - Scan for a device matching the interface class `0xFF`, subclass `0xCF`, and
     protocol `0x66`.
   - If a serial number was specified, ensure the device's serial number matches.
   - Claim the interface and identify the IN and OUT endpoint addresses.
2. **Transmit**:
   - Marshal the `UsbTpmHeader` for `MS_SIM_TPM_SEND_COMMAND`.
   - Send the header transfer via a bulk OUT transfer.
   - Send the TPM command payload transfer via a bulk OUT transfer.
3. **Receive**:
   - Receive the `UsbTpmHeader` via a bulk IN transfer.
   - Verify the `header` field matches `TPM_` and the `identifier` field matches
     `_RSP`.
   - Check the `status` field.
   - If `STATUS_SUCCESS`, receive the TPM response payload via a bulk IN transfer
     based on the `length` in the header.
4. **Finalize**:
   - Release the USB interface.
   - Close the device handle and deinitialize `libusb`.

Since this protocol communicates with a device that behaves similarly to a TPM,
there is no need for an out-of-band "platform socket" for state changes.
Platform-level commands (like `MS_SIM_NV_ON`) are sent as standard protocol
requests.

## USB Device Implementation

The device side of the `usb-test-protocol` is typically implemented as a state
machine that handles incoming bulk transfers and control requests.

### State Machine

A typical implementation would use the following states:

1.  **`Idle`**: The device is waiting for a request from the host. It polls the
    Bulk OUT endpoint for a 16-byte `UsbTpmHeader`.
2.  **`ReceivingHeader`**: The device is accumulating the 16 bytes of the header.
    If the header is valid (correct FourCCs) and the command is recognized:
    - If `length > 0`, it transitions to **`ReceivingPayload`**.
    - If `length == 0`, it transitions to **`Processing`**.
    If the header is invalid or the command is unknown, it responds with an
    appropriate error status and transitions back to **`Idle`**.
3.  **`ReceivingPayload`**: The device accumulates `length` bytes of the payload
    from the Bulk OUT endpoint. Once the full payload is received, it
    transitions to **`Processing`**.
4.  **`Processing`**: The device executes the TPM command or protocol operation.
    During this time, it may NAK any further Bulk OUT packets. Once execution
    is complete, it prepares the response and transitions to
    **`SendingResponse`**.
5.  **`SendingResponse`**: The device sends the 16-byte response header followed
    by the response payload on the Bulk IN endpoint. Once the full transfer is
    complete (including any necessary ZLP), it transitions back to **`Idle`**.

### Control Request Handling

The device must handle the `Vendor` interface requests for `setLocality` and
`cancel`. These requests can arrive at any time and should be handled
asynchronously to the bulk transfer state machine.

- **`setLocality`**: Updates the internal state of the device to use the
  specified locality for subsequent TPM commands.
- **`cancel`**: Signals the device to abort the current `Processing` state if
  possible. If a command is canceled, the device should still return a valid
  TPM response (e.g., `TPM_RC_CANCELLED`) or a protocol-level error status.
