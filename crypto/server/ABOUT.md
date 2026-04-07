# About `crypto/server`

The implementation of the crypto service. It dispatches incoming IPC requests to specific handlers and interfaces with the **OpenTitan Crypto (`OtCrypto`)** library.

- **server.rs**: Main server dispatch function based on `Opcode`.
- **asymmetric.rs**: Helper functions for executing asymmetric crypto operations (ECDSA, DICE).
- **digest.rs**: Helper functions for executing digest operations (SHA2), supporting up to 4 concurrent contexts.
- **lib.rs**: Root of the library.
