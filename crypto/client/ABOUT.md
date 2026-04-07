# About `crypto/client`

A library for applications to access crypto services via IPC. It provides a high-level Rust interface by wrapping underlying IPC calls.

- **backend.rs**: Implements the `Backend` trait by sending messages over an IPC handle.
- **sha2.rs**: High-level interface for SHA2 hashing (256, 384, 512).
- **ecdsa.rs**: High-level interface for ECDSA P-256 and DICE P-256 operations.
- **util/**: Helpers for constructing asymmetric and digest requests.
- **lib.rs**: Root of the library.
- **util.rs**: Utility submodule.
