# About `crypto/traits`

Core abstractions (Rust traits) that define the cryptographic interfaces. These traits decouple operations from their specific implementations (e.g., IPC vs. local).

- **asymmetric.rs**: Traits for asymmetric algorithms like `Sign`, `Verify`, and `KeyPairGen`.
- **backend.rs**: Defines the `Backend` trait representing a cryptography provider.
- **digest.rs**: Traits for digest and MAC algorithms (`DigestInit`, `DigestUpdate`, `DigestFinal`).
- **error.rs**: Error definitions.
- **lib.rs**: Root of the library.
