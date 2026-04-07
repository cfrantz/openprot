# About `crypto/common`

Shared definitions used for IPC communication between the crypto client and server.

- **keytypes.rs**: Defines memory-stable structures for keys and signatures (ECDSA, RSA, etc.) using `zerocopy` to ensure safety across FFI/IPC boundaries.
- **opcode.rs**: A central registry of operation codes (Opcodes) used to identify and dispatch requests.
- **lib.rs**: Root of the library.
