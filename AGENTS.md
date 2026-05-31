# AGENTS.md: AI Collaboration Guide for OpenPRoT

This document is a machine-readable configuration file for AI agents, developers, and LLM-assisted tools (such as Gemini Coder, Cider, and Jetski) working within this repository. 

Adhere strictly to all rules, conventions, and architectural boundaries detailed below.

---

## 1. Project Overview & Purpose

*   **Primary Goal**: OpenPRoT is a specialized framework for building secure, robust **Root-of-Trust (RoT) firmware stacks**. 
*   **Target Domain**: Security-critical embedded hardware, cryptographic operations, and low-level chip life-cycle enablement.
*   **Active Platform Focus**: Enablment and verification of the **OpenTitan Earlgrey** chip target.
*   **Environment Isolation Constraint**: 
    > [!CRITICAL]
    > **THIS IS NOT A GOOGLE3 PROJECT.**
    > You MUST NOT use, reference, or import any internal `google3` dependencies, libraries, or APIs. 
    > All infrastructure is completely open-source/external and uses standard external toolchains.

---

## 2. Technology Stack & Architecture

### Languages
*   **Rust (Edition 2024)**: Core codebase language.
    *   **Strict Bare-Metal Constraints**: The target build environment runs in a raw `no_std` context with **zero heap allocations** allowed.
*   **Starlark**: Used for Bazel build rules and configurations.
*   **Python**: Scripting, signing, and workflow wrapper runner utilities.
*   **Assembly**: Low-level CPU initialization and early boot sequences (target RISC-V 32-bit architecture).

### Microkernel
*   **Pigweed's Maize (`pw_kernel`)**: High-reliability microkernel providing:
    *   System process separation, thread scheduling, and userspace memory isolation.
    *   Synchronous IPC via Channel Objects.
    *   Interrupt service configurations.
    *   System images declared declaratively via `system.json5` configuration files and built with Pigweed's `system_image()` macros.

### Build System
*   **Bazel / Bazelisk**: The authoritative build and test engine. 
    *   *Always* invoke commands using `bazelisk` locally to guarantee compiler toolchain reproducibility and automatic Bazel version alignment.

### Key Directories
*   [`/target`](file:///usr/local/google/home/cfrantz/src/openprot/hwe-new/target): Target-specific platforms and chip mappings.
    *   [`/target/earlgrey`](file:///usr/local/google/home/cfrantz/src/openprot/hwe-new/target/earlgrey): Linker files, drivers, hardware-specific entrypoints, and test setups for OpenTitan.
    *   [`/target/earlgrey/signing`](file:///usr/local/google/home/cfrantz/src/openprot/hwe-new/target/earlgrey/signing): Cryptographic tokens, YAML signatures, and keys.
*   [`/hal`](file:///usr/local/google/home/cfrantz/src/openprot/hwe-new/hal): Hardware Abstraction Layer traits for I/O (`async`, `blocking`, `nb`).
*   [`/drivers`](file:///usr/local/google/home/cfrantz/src/openprot/hwe-new/drivers): USART, GPIO, and other chip-level drivers.
*   [`/openprot`](file:///usr/local/google/home/cfrantz/src/openprot/hwe-new/openprot): Core firmware, kernel wrappers, and state-machine logic.
*   [`/services`](file:///usr/local/google/home/cfrantz/src/openprot/hwe-new/services): Higher-level system services: MCTP, secure storage, and hardware telemetry.
*   [`/docs`](file:///usr/local/google/home/cfrantz/src/openprot/hwe-new/docs): `mdbook` source documentation (build using `bazelisk build //docs`).

---

## 3. Coding Conventions & Style Guide

### Rust Formatting & Style
*   All Rust code must be formatted with the project's standard formatter config:
    ```bash
    ./pw format
    ```
*   **Crate Configuration**: Must always preserve the `#![no_std]` directive at the crate root. No standard-library (`std`) dependencies are allowed.

### Forbidden Patterns & Alternatives

| Forbidden Pattern | Required Alternative | Context & Rationale |
| :--- | :--- | :--- |
| `value.unwrap()` | `match` or `if let` matching | Avoid panic paths in security-critical firmware. |
| `result.expect("msg")` | Map error explicitly using `?` | Panics in root firmware lead to permanent lockup. |
| `collection[index]` | `collection.get(index)` | Slice/array indexing must be bounds-checked at compile/runtime. |
| `a + b` (integers) | `a.checked_add(b)`, `saturating_add`, or `wrapping_add` | Prevent silent integer overflows in arithmetic blocks. |
| `ptr.read()` | `ptr.read_volatile()` | Volatile access is required for MMIO registers. |
| `Vec<T>`, `HashMap<K, V>` | Stack arrays `[T; N]` or `heapless::Vec<T, N>` | Zero dynamic heap memory allocation constraint. |
| `String` | `heapless::String<N>` or `&str` | No heap-allocated string buffers allowed. |
| `Box<T>` | Stack variables or direct `&mut T` reference | No heap boxing allowed. |

### Security & Hardware abstraction
*   **Volatile Accesses**: Register reading and writing must use explicit volatile functions (`read_volatile` / `write_volatile`) and must abstract through the official [HAL](file:///usr/local/google/home/cfrantz/src/openprot/hwe-new/hal) traits rather than raw register maps.
*   **Constant-time Crypto**: Use the `subtle` crate for secret comparisons to prevent timing side-channel attacks.
*   **Zeroization**: Secure material (keys, seed phrases, internal hashes) must be explicitly cleared using the `zeroize` crate immediately after usage.
*   **Unsafe Blocks**: Every `unsafe` block must be prefixed with a `// SAFETY:` comment explicitly verifying that all safety conditions are maintained.

---

## 4. Common Development Commands

OpenPRoT integrates Pigweed's `pw` wrapper, which routes commands to Bazel via the custom workflows defined in [`workflows.json`](file:///usr/local/google/home/cfrantz/src/openprot/hwe-new/workflows.json).

### Build Tasks
*   **Build the entire tree**:
    ```bash
    bazelisk build //...
    ```
*   **Build specific OpenTitan target components**:
    ```bash
    bazelisk build //target/earlgrey/...
    ```
*   **Build the OpenTitan IPC test**:
    ```bash
    bazelisk build //target/earlgrey/ipc/user:ipc
    ```

### Test Tasks
*   **Run CI unit tests (excluding real hardware / simulator requirements)**:
    ```bash
    ./pw ci
    ```
*   **Run unit tests under OpenTitan Earlgrey CW310 board simulation**:
    ```bash
    bazelisk test --test_output=all --cache_test_results=no //target/earlgrey/unittest_runner:hyper310_test
    ```
*   **Run unit tests under OpenTitan Earlgrey CW340 board simulation**:
    ```bash
    bazelisk test --test_output=all --cache_test_results=no //target/earlgrey/unittest_runner:hyper340_test
    ```
*   **Run IPC user-tests under Verilator hardware simulation**:
    ```bash
    bazelisk run //target/earlgrey/ipc/user:ipc_runner_verilator
    ```
*   **Run the full upstream Pigweed bump compatibility sweep**:
    ```bash
    ./pw upstream_pigweed
    ```

### Code Quality & Formatting Tasks
*   **Format the workspace (Rust, Starlark, C++, Python)**:
    ```bash
    ./pw format
    ```
*   **Run the full formatting, license check, header validation, and clippy sweep**:
    ```bash
    ./pw presubmit
    ```

### Documentation
*   **Build the `mdbook` docs**:
    ```bash
    bazelisk build //docs
    ```
*   **Serve the docs locally on `http://localhost:8000`**:
    ```bash
    bazelisk run //docs:serve
    ```

---

## 5. Critical Constraints & Safety Guardrails

*   **NO GOOGLE3 SYMBOLS**: Under no circumstances should you propose adding dependencies or files that use google3 structures or paths.
*   **UNSAFE REVIEW MANDATE**: If adding or editing `unsafe` blocks, you MUST explicitly document the reasoning with the `// SAFETY:` block, verifying that no out-of-bounds, data-race, or dereference crashes can occur.
*   **CRYPTOGRAPHIC KEY INTEGRITY**: Do not alter, modify, or create configuration tokens or public keys in [`/target/earlgrey/signing`](file:///usr/local/google/home/cfrantz/src/openprot/hwe-new/target/earlgrey/signing) without explicit user authorization.

---

## 6. AI Collaboration & Lifecycle

*   **Continuous Synchronization**: If you introduce a new helper crate, system constraint, platform flag, or target module, you MUST proactively suggest updates to this `AGENTS.md` (and the mirror `GEMINI.md`) inside your PR or CL description.
*   **Presubmit Requirement**: Always run `./pw presubmit` before declaring a coding task completed.

---

*This document was co-authored by Jetski and a Google DeepMind researcher to structure precise collaboration with AI models.*
