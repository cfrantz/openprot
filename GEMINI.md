# GEMINI.md: AI Collaboration Guide for OpenPRoT

This document is a machine-readable configuration file for the Gemini CLI, Cider, and Jetski agents working in this repository. It establishes our development persona, technology constraints, custom commands, and safety guardrails.

---

## 1. Context Linkage & Single Source of Truth

> [!NOTE]
> This project adheres to both the internal Gemini CLI context specifications and the modern, provider-agnostic standard.
> All rules, stack specifications, commands, and constraints defined here are synchronized with our core agent configuration:
> **[AGENTS.md](file:///usr/local/google/home/cfrantz/src/openprot/hwe-new/AGENTS.md)**.

---

## 2. Project Overview & Purpose

*   **Primary Goal**: OpenPRoT is a framework for building secure **Root-of-Trust (RoT) firmware stacks**. 
*   **Target Domain**: Security-critical embedded hardware, low-level chip lifecycle enablement, and secure cryptographic boot structures.
*   **Active Platform Focus**: Enablement and verification of the **OpenTitan Earlgrey** chip target.
*   **Environment Isolation Constraint**: 
    > [!CRITICAL]
    > **THIS IS NOT A GOOGLE3 PROJECT.**
    > Do NOT import, reference, or use any `google3` dependencies, libraries, or APIs. 
    > All infrastructure is completely open-source/external and uses standard external toolchains.

---

## 3. Technology Stack & Architecture

### Languages & Triples
*   **Rust (Edition 2024)**: Core codebase language.
    *   **Context Constraints**: Target platform runs in bare-metal `no_std` context with **zero heap allocations** allowed.
    *   **Target Triple**: `riscv32imc-unknown-none-elf` for OpenTitan, and host platform targets.
*   **Starlark**: Bazel build rules (`BUILD.bazel` and module setup files).
*   **Python**: Scripts, utilities, and helper tools.
*   **Assembly**: Low-level CPU initialization and early boot stages.

### Microkernel & Core Platform
*   **Pigweed's Maize (`pw_kernel`)**: Low-level microkernel providing process isolation, threads, interrupt setup, and synchronous channel-based IPC.
*   **Pigweed Libraries**: Integrated build, logging, status types (`pw_log`, `pw_status`).
*   **Build System**: Bazel / Bazelisk.

### Key Directories
*   [`/target`](file:///usr/local/google/home/cfrantz/src/openprot/hwe-new/target): Mappings and device tests.
    *   [`/target/earlgrey`](file:///usr/local/google/home/cfrantz/src/openprot/hwe-new/target/earlgrey): Linkers, drivers, entry routines, simulation targets.
    *   [`/target/earlgrey/signing`](file:///usr/local/google/home/cfrantz/src/openprot/hwe-new/target/earlgrey/signing): Cryptographic signing setups, tokens, and keys.
*   [`/hal`](file:///usr/local/google/home/cfrantz/src/openprot/hwe-new/hal): Hardware Abstraction Layer traits for secure stack interfaces.
*   [`/drivers`](file:///usr/local/google/home/cfrantz/src/openprot/hwe-new/drivers): Low-level hardware drivers.
*   [`/openprot`](file:///usr/local/google/home/cfrantz/src/openprot/hwe-new/openprot): Central firmware lifecycle management.

---

## 4. Coding Conventions & Forbidden Patterns

### Forbidden Patterns & Alternatives

| Forbidden Pattern | Required Alternative | Rationale & Impact |
| :--- | :--- | :--- |
| `value.unwrap()` | `match` or `if let` matching | Prohibited. A panic in root firmware leads to permanent chip lockup. |
| `result.expect("msg")` | Map error explicitly using `?` | Prohibited. Handle error cases gracefully with robust states. |
| `collection[index]` | `collection.get(index)` | Bounds violations trigger immediate panic. Use safe lookups. |
| `a + b` (integers) | `a.checked_add(b)`, `saturating_add`, or `wrapping_add` | Numeric overflow is a key target for remote buffer exploits. |
| `ptr.read()` | `ptr.read_volatile()` | Required for accurate memory-mapped I/O (MMIO) hardware. |
| `Vec<T>`, `HashMap<K, V>` | Stack arrays `[T; N]` or `heapless::Vec<T, N>` | Heap operations are banned. Stack bounds must remain small. |
| `String` | `heapless::String<N>` or `&str` | No heap-allocated string manipulation. |

### Security Guardrails
*   **MMIO Access**: Raw pointer accesses must be scoped behind [HAL](file:///usr/local/google/home/cfrantz/src/openprot/hwe-new/hal) traits.
*   **Constant-time**: Always use the `subtle` crate when validating sensitive variables (e.g. passwords, hash values, signatures) to prevent timing side-channel attacks.
*   **Zeroization**: Clear intermediate secrets in memory using the `zeroize` crate immediately after usage.
*   **Unsafe Invariants**: Every `unsafe` block must state a `// SAFETY:` reason explaining why compiler invariants are met.

---

## 5. Common Development Commands

OpenPRoT uses Pigweed's workflow manager `./pw`, driving tests and compiles using `bazelisk`.

### Tasks & Compiles
*   **Workspace Build**:
    ```bash
    bazelisk build //...
    ```
*   **OpenTitan Target Build**:
    ```bash
    bazelisk build //target/earlgrey/...
    ```
*   **Build the OpenTitan IPC application**:
    ```bash
    bazelisk build //target/earlgrey/ipc/user:ipc
    ```

### Simulations & Testing
*   **Standard CI validation suite**:
    ```bash
    ./pw ci
    ```
*   **Run Earlgrey board tests (CW310 board simulation)**:
    ```bash
    bazelisk test --test_output=all --cache_test_results=no //target/earlgrey/unittest_runner:hyper310_test
    ```
*   **Run Earlgrey board tests (CW340 board simulation)**:
    ```bash
    bazelisk test --test_output=all --cache_test_results=no //target/earlgrey/unittest_runner:hyper340_test
    ```
*   **Run Verilator simulator test framework**:
    ```bash
    bazelisk run //target/earlgrey/ipc/user:ipc_runner_verilator
    ```
*   **Upstream bump validation**:
    ```bash
    ./pw upstream_pigweed
    ```

### Formatters & Linters
*   **Format the codebase**:
    ```bash
    ./pw format
    ```
*   **Run formats, clippy checks, and validation suites**:
    ```bash
    ./pw presubmit
    ```

### Documentation
*   **Build mdbook documentation**:
    ```bash
    bazelisk build //docs
    ```
*   **Locally serve docs website**:
    ```bash
    bazelisk run //docs:serve
    ```

---

## 6. Constraints & Safety Guardrails

*   **UNSAFE REVIEWS**: Do not insert or update `unsafe` code without a corresponding `// SAFETY:` block.
*   **CRYPTOGRAPHIC SECURE STATE**: Do not modify files inside [`/target/earlgrey/signing`](file:///usr/local/google/home/cfrantz/src/openprot/hwe-new/target/earlgrey/signing) (cryptographic setups) without direct developer requests.
*   **NO GOOGLE3 SYMBOLS**: Proactively reject any imports containing `google3/` paths, targets, or constructs.

---

*This document was co-authored by Jetski and a Google DeepMind researcher to ensure a highly standardized and secure partnership between human developers and AI coding agents.*
