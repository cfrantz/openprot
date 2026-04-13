# Null Cryptography Implementation

This directory contains a "null" implementation of the TPM cryptography API.
It is intended for testing and development purposes where actual cryptographic operations are not required or desired.

## Implementation Details

The null implementation adheres to the following principles:
- **Initialization:** All subsystem initialization and startup functions return `true`.
- **Hashing:** All hashing functions return success with zero-length outputs or zero digest sizes. `HashDef` structures report zero sizes.
- **Random Number Generation:** The random number generator returns buffers filled with zeros.
- **Symmetric Cryptography:** Encryption and decryption operations return `TpmRc::Success` without modifying the output buffers (except for block size and key validation checks which return zero and Success respectively).
- **Asymmetric Cryptography (RSA & ECC):** All key generation, encryption, decryption, signing, and verification operations return `TpmRc::Success`. Point multiplication and other ECC operations also return `TpmRc::Success`.

## Use Cases

- **Unit Testing:** When testing TPM logic that doesn't depend on the correctness of cryptographic results.
- **Performance Prototyping:** To measure the overhead of the TPM software stack excluding cryptographic processing time.
- **Bootstrap/Bring-up:** During early stages of development when a real crypto provider is not yet integrated.

**WARNING:** This implementation provides NO SECURITY. It should NEVER be used in a production environment.
