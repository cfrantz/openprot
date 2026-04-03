# About `services/tpm`

The code in this subdirectory is for integrating the TCG-TPM codebase into
OpenPRoT firmware.

The TCG-TPM is a C codebase; we have written a rust wrapper abstraction that
allows us to cleanly replace the TPM platform interface library and the
TPM crypto library with libraries appropriate for OpenPRoT targets.
