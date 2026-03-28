# Gemini Context: tpm2-tss

This repository hosts source code for the OpenPRoT project.  The OpenPRoT
project aims to develop a toolkit (a set of useful libraries and reference
implementations) for building root-of-trust firmware.

The firmware is primarily written in rust, but we do allow C code in the
codebase.  The firmware runs on the pigweed Maize kernel, a rust microkernel
with static configuration.

This project uses bazel as its build system and declares pigweed as an upstream
dependency.  In addition to using the pigweed Maize kernel, this project
utilizes several other pigweed resources. Primarily these include the toolchain
configuration, workflow automation and pigweed's linters and formatting
checkers.

## Communication & Clarification
- If any instruction or architectural direction is ambiguous, you **must** ask the user for clarification before proceeding with the implementation. Never make assumptions that lead to implementing logic with uncertainty (e.g., leaving questions in code comments).
- **Commit Approval:** Do not perform git commits unless specifically approved by the user or if the user temporarily countermands this instruction.
- **Commit Format:** When committing, you **must** use the `--signoff` (or `-s`) flag. Additionally, you **must** include a trailer identifying the AI assistant by using the `--trailer "AI-assistant: Gemini"` flag (e.g., `git commit -s --trailer "AI-assistant: Gemini" ...`). Do not override the commit author's email address.
