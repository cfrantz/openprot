# About `crypto`

This is the toplevel subdirectory of the crypto interfaces, implementing an IPC-based client-server architecture for cryptographic services.

The following subdirectories exist:
- **client**: A library for applications to access crypto services via IPC.
- **common**: Shared definitions used for IPC communication, including key types and opcodes.
- **server**: The implementation of the crypto service, typically running in a more privileged or hardware-interfacing context.
- **traits**: Core abstractions (Rust traits) that decouple cryptographic operations from their implementation.

## Capabilities
- **Hashing**: Supports SHA2-256, SHA2-384, and SHA2-512 with a streaming interface.
- **Asymmetric Crypto**: Support for ECDSA P-256 (keygen, sign, verify) and DICE P-256.
- **Key Management**: Distinguishes between software-backed and hardware-backed keys.
