# Hardware Enablement (HWE) Firmware Development Plan: OpenTitan Earlgrey

This document establishes a refined, prioritized implementation blueprint for developing the Hardware Enablement (HWE) bare-metal firmware stack for the **OpenTitan Earlgrey** target chip, using the **Pigweed Maize (`pw_kernel`)** microkernel.

This plan incorporates a zero per-call overhead Rust modular design: **Task-Specific Static Loggers linked via `extern "Rust"` symbol resolution**. Each task defines a single `#[no_mangle] static UTIL_ZFMT_LOGGER: FlatAdapter<IpcLogger, LOG_BUFFER_SIZE>` const-initialized with its IPC handle. The generic `//util/zfmt` library declares a matching `extern "Rust" { static UTIL_ZFMT_LOGGER }` that logging macros reference directly — `LOG_BUFFER_SIZE` bytes of `.bss` per task binary, zero flash cost, and no per-call stack allocation.

---

## 1. Architectural & Strategic Invariants

To maintain high standards of safety, isolation, and performance in the Root of Trust context:

1. **Structured IPC Logging (`zfmt` Baseline)**:
   - All userspace runtime logging MUST use the standalone binary `zfmt` event package (`@zfmt//zfmt`).
   - **Strict Ban on `pw_log`**: Under no circumstances should standard runtime firmware tasks (e.g., `sysmgr`, `flash_server`, `platform`) invoke `pw_log` at runtime.
   - **Test Exception**: Standard `pw_log` is permitted inside target test runners (`target/earlgrey/tests/...`) to keep assertions simple.
   - **Initial Milestone — Text Output Mode**: The initial implementation uses `zfmt` text output mode. Binary TLV framing, `StreamStart` sync frames, and `EventHeader` sequence stamping are deferred to a later milestone once the text-mode pipeline is validated end-to-end.
2. **Task-Specific Static Loggers (`extern "Rust" static`)**:
   - **Task Static Definition**: Each userspace binary task defines a single static logger, const-initialized with its generated IPC handle:
     ```rust
     #[no_mangle]
     static UTIL_ZFMT_LOGGER: FlatAdapter<IpcLogger, LOG_BUFFER_SIZE> =
         FlatAdapter::new(IpcLogger::new(handle::LOG_IPC_HANDLER));
     ```
   - **Link-Time Symbol Resolution**: The generic library `//util/zfmt` declares a matching `extern "Rust"` static that logging macros dereference:
     ```rust
     extern "Rust" {
         static UTIL_ZFMT_LOGGER: FlatAdapter<IpcLogger, LOG_BUFFER_SIZE>;
     }
     ```
     A safe wrapper `pub fn logger() -> &'static FlatAdapter<IpcLogger, LOG_BUFFER_SIZE>` confines the single `unsafe` deref to `//util/zfmt` internals, so call sites remain safe.
   - **Memory Properties**: `UTIL_ZFMT_LOGGER` occupies `LOG_BUFFER_SIZE` bytes of `.bss` per task binary (zero-initialized at boot, zero flash cost). There is no per-call stack allocation and no runtime initialization required.
   - **Sync Safety**: All tasks are currently single-threaded. If `FlatAdapter` requires an explicit `unsafe impl Sync` due to interior mutability, it is safe to assert in `//util/zfmt` under this invariant.
3. **Target-Specific Clock Abstractions**:
   - `//util/zfmt` uses a `mod clock` resolved at build time by Bazel `select` in `util/zfmt/BUILD.bazel`: on Earlgrey it references `//target/earlgrey/util:clock` by label; on host/default it includes `clock_host.rs` (returns 0) from within `//util/zfmt`.
   - The Earlgrey clock implementation lives in `//target/earlgrey/util/clock.rs` alongside the other target utilities. This keeps all `#[cfg]` out of `//util/zfmt`; new targets add a clock implementation in their own util package and a `select` arm in `//util/zfmt/BUILD.bazel` without touching `lib.rs`.
4. **Strict Process Boundaries & MMIO Isolation**:
   - **No Direct Low-level Console Writing**: Tasks must never bypass isolation using `extern "C" system_lowlevel_console_write`. 
   - **The USB & Logging Task** (running `usbdfu.rs`) is the exclusive owner of `usbdev` and `Uart0`.
   - **Timing/Hand-off Invariant**: When the userspace starts, the microkernel will have completed its early boot logging, allowing the USB task to assume exclusive control of `Uart0` registers without collision.
   - **IPC Stream Aggregation**: Every other task streams its raw binary log events via standard `syscall::channel_transact` over an IPC channel back to the central USB task.
5. **Dual-Output Log Ring Buffer**:
   - The USB task maintains a single circular byte buffer (`LOG_RING_SIZE` bytes) that all enabled output backends consume from independently.
   - Each enabled backend owns its own read cursor; the write cursor advances on every received frame.
   - **Frame-granular drop-oldest eviction**: when a new frame does not fit, the oldest complete frame(s) are evicted until there is room. The most recent events are always preserved; a slow or absent consumer silently falls behind.
   - **Sequence number stamping**: the USB task holds a global `u32` sequence counter. On each IPC log frame received it increments the counter and stamps it into the frame's `EventHeader` before writing to the ring buffer. Host-side decoders detect dropped events via gaps in sequence numbers — no synthetic drop-event frame is needed.
   - **Non-blocking drains**: each output's `drain()` writes what the hardware can accept right now (UART TX FIFO bytes; one CDC-ACM bulk packet if an endpoint buffer is free) and returns immediately. Drains are called each event loop iteration **and** between DFU flash page writes to prevent starvation during long flash operations.
   - **Feature-gated outputs**: `log_uart` and `log_cdc_acm` Rust features (controlled via Bazel `select` on `crate_features`) gate the UART and CDC-ACM output modules. The ring buffer and write path compile unconditionally; disabling both outputs produces a silent build.
   - **Buffer sizing**: initial size 4 KB; to be revisited after DFU page-write latency is characterized.
6. **Maximized Branch Reuse (`hwe2` Refactoring)**:
   - We will refactor and reuse components from the branch `hwe2`: `eflash_driver.rs`, `//services/flash` client/server logic, retention RAM utility (`ret_ram.rs`), ROM boot status utilities, core DFU logic (`usbdfu.rs`), and `sysmgr`.
6. **No Storage & Telemetry**:
   - `//services/storage` and `//services/telemetry` are ignored for the current milestones.
7. **Consolidated Flash Service**:
   - The flash service is kept entirely within `//services/flash` containing both the `client.rs` and `server.rs` modules under two distinct targets.

---

## 2. Target Directory Map (Refined)

```
├── hal/
│   └── blocking/
│       └── src/
│           └── lib.rs                     # Generic blocking traits (e.g. blocking flash)
├── services/
│   └── flash/                             # Consolidated Flash Service (client & server)
│       ├── BUILD.bazel
│       ├── opcode.rs                      # Flash IPC transaction opcodes
│       ├── client.rs                      # Client IPC helper
│       └── server.rs                      # Generic Flash partition server logic
├── util/
│   └── zfmt/                              # Generic project-wide zfmt IPC Logger
│       ├── BUILD.bazel                    # select pulls in //target/earlgrey/util:clock or clock_host.rs
│       ├── lib.rs                         # IpcLogger, extern static, logger(), log macros
│       └── clock_host.rs                 # Default/host clock implementation (returns 0)
├── target/
│   └── earlgrey/
│       ├── drivers/
│       │   ├── BUILD.bazel
│       │   ├── eflash_driver.rs           # Low-level Embedded Flash driver
│       │   ├── uart_driver.rs             # UART TX/RX driver (to be implemented in Phase 1)
│       │   ├── uart_receiver.rs           # Dedicated non-blocking UART rx buffer
│       │   └── usb_driver.rs              # Physical USB device driver
│       ├── util/
│       │   ├── BUILD.bazel
│       │   ├── lib.rs                     # Unified utility exposure
│       │   ├── boot_svc.rs                # Bootloader config & handoff definitions
│       │   ├── ret_ram.rs                 # Retention RAM layout & state checks
│       │   ├── rom_error.rs               # Decoders for ROM boot stages
│       │   ├── timer.rs                   # Exclusive target Earlgrey timer utility
│       │   ├── mubi.rs                    # Multi-bit hardened boolean primitives
│       │   └── clock.rs                   # zfmt clock impl; referenced by //util/zfmt via label
│       ├── services/
│       │   ├── sysmgr/
│       │   │   ├── BUILD.bazel
│       │   │   ├── client.rs              # Sysmgr IPC Client helper
│       │   │   └── server.rs              # Sysmgr Supervisor Service
│       │   ├── usbdfu/
│       │   │   ├── BUILD.bazel
│       │   │   ├── log_ring.rs            # Ring buffer, sequence stamping, read cursors
│       │   │   ├── uart_output.rs         # feature="log_uart": non-blocking UART drain
│       │   │   ├── cdc_output.rs          # feature="log_cdc_acm": non-blocking CDC-ACM drain
│       │   │   └── dfu.rs                 # USB DFU protocol state machine
│       │   └── platform/
│       │       ├── BUILD.bazel
│       │       └── platform.rs            # Pinmux, GPIO, board multiplexer logic
│       └── firmware/
│           └── hwe/
│               ├── BUILD.bazel
│               ├── system.json5           # Central HWE process architecture descriptor
│               ├── target.rs              # HWE kernel configuration
│               ├── sysmgr.rs              # Entry point: wait_group assembly, IPC dispatch
│               ├── platform.rs            # Entry point: setup, outer main loop
│               ├── flash_server.rs        # Entry point: setup, outer main loop
│               └── usbdfu.rs              # Entry point: setup, outer main loop
```

---

## 3. Process & IPC Communication Map

The Mermaid diagram below represents the corrected userspace boundaries, focusing on the centralized log aggregation path.

```mermaid
graph TB
    subgraph Userspace Tasks (Isolated Processes)
        SysMgr["System Manager (sysmgr)"]
        UsbTask["USB & Logging Task (usbdfu)<br><i>Owns Uart0 & usbdev MMIO</i>"]
        PlatformTask["Platform Task (platform)"]
        FlashService["Flash Server (flash_server)"]
    end

    subgraph Physical Hardware Peripherals
        USB_HW[("USB Device Controller<br>(usbdev)")]
        UART0_HW[("UART 0 Controller<br>(uart0)")]
        FLASH_HW[("Flash Controller<br>(flash_ctrl)")]
    end

    %% IPC Logging pathways (Forwarding zfmt bytes)
    SysMgr -- "IPC: Stream zfmt bytes via task static LOGGER" --> UsbTask
    PlatformTask -- "IPC: Stream zfmt bytes via task static LOGGER" --> UsbTask
    FlashService -- "IPC: Stream zfmt bytes via task static LOGGER" --> UsbTask

    %% Regular IPC pathways
    SysMgr -- "IPC: Handoff States" --> UsbTask
    UsbTask -- "IPC: Read/Write Block" --> FlashService

    %% Direct Hardware Mappings (Protected MMIO)
    UsbTask -.->|Exclusive MMIO Map| USB_HW
    UsbTask -.->|Exclusive MMIO Map| UART0_HW
    FlashService -.->|Exclusive MMIO Map| FLASH_HW
```

---

## 4. Phase-by-Phase Priority Execution Plan

### Phase 1: The Modular zfmt IPC Logging System (Baseline)
*Goal: Implement the Earlgrey UART driver, set up the generic `//util/zfmt` crate with extern resolution, Earlgrey timer utility, and the USB task log aggregator with text-mode output.*

1. **Earlgrey Timer Utility (`target/earlgrey/util/timer.rs`)**:
   - Abstract access to `registers::rv_timer::RvTimer` registers.
   - Expose a clean struct `EarlGreyTimer` that other utilities/loggers can use to safely fetch standard 64-bit tick counts.
2. **Earlgrey UART Driver (`target/earlgrey/drivers/uart_driver.rs`)**:
   - Implement a proper UART TX/RX driver against the Earlgrey UART register interface. No equivalent currently exists in `//target/earlgrey/drivers`.
   - Expose non-blocking TX (write bytes to FIFO, return bytes accepted) and RX interfaces sufficient for use by the `uart_output` drain in `//target/earlgrey/services/usbdfu`.
3. **Generic IPC Logger with Link-Time Clock & Symbol Resolution (`//util/zfmt`)**:
   - Create `util/zfmt/lib.rs` and `util/zfmt/BUILD.bazel`.
   - Use Bazel `select` inside `util/zfmt/BUILD.bazel` to depend on `//target/earlgrey/util` when compiling for the Earlgrey platform.
   - Implement the `IpcLogger` struct that **directly holds the `u32` handle** of the task's IPC logging channel:
     ```rust
     pub struct IpcLogger {
         pub handle: u32,
     }
     ```
   - Write `clock.rs` in `//target/earlgrey/util` (calls `EarlGreyTimer::read_ticks`) and `clock_host.rs` in `//util/zfmt` (returns 0). Use Bazel `select` in `//util/zfmt/BUILD.bazel` to reference `//target/earlgrey/util:clock` by label on Earlgrey, falling back to `clock_host.rs` on host/default.
   - Declare `extern "Rust" { static UTIL_ZFMT_LOGGER: FlatAdapter<IpcLogger, LOG_BUFFER_SIZE>; }` inside `util/zfmt/lib.rs` and expose a safe `logger()` accessor.
   - Export global logging macros (`log_info!`, etc.) that call `logger()` directly — no per-call stack allocation.
4. **Log Aggregation Service (`//target/earlgrey/services/usbdfu`)**:
   - Implement `log_ring.rs`: the `LogRingBuffer` struct with write cursor, per-consumer read cursors, and frame-granular drop-oldest eviction. Sequence number stamping is deferred until binary mode is adopted.
   - Implement `uart_output.rs` (feature `log_uart`) and `cdc_output.rs` (feature `log_cdc_acm`) as non-blocking drain functions that advance their respective cursors by as much as hardware will accept per call.
   - Implement `dfu.rs`: USB DFU protocol state machine.
   - The thin entry point `//target/earlgrey/firmware/hwe/usbdfu.rs` maps the `LOGGING_CHANNEL` handle and runs the outer event loop: receive IPC frames → `log_ring::write_frame` → `uart_output::drain` → `cdc_output::drain` → `handle_dfu_events` → wait.
5. **Verification Task**:
   - Implement an integration test verifying that human-readable zfmt text events appear on `Uart0`. Binary frame validation (`StreamStart::ZFMT_TAG`, `EventHeader::ZFMT_TAG`, sequence numbers) is deferred to the binary-mode milestone.

### Phase 2: Embedded Flash & USB DFU (Firmware Self-Update)
*Goal: Refactor flash drivers, services, and DFU transitions utilizing the new generic log pipeline.*

1. **Low-Level Flash Driver (`target/earlgrey/drivers/eflash_driver.rs`)**:
   - Pull and refactor the eflash driver from branch `hwe2`.
2. **Consolidate the Flash Service (`services/flash`)**:
   - Pull `services/flash` from `hwe2`.
   - Expose `:client` and `:server` targets in `services/flash/BUILD.bazel`.
   - Refactor `services/flash/server.rs` replacing standard `pw_log` calls with Phase 1 generic `util_zfmt` logger macros.
3. **USB DFU & Logging Task Integration (`target/earlgrey/firmware/hwe/usbdfu.rs`)**:
   - Port `usbdfu.rs` from `hwe2`.
   - Merge DFU and log aggregation loop.
   - Refactor DFU transition logs (`DNLOAD`, `UPLOAD`, `MANIFEST`) to emit binary `zfmt` events.

### Phase 3: System Manager Daemon & Process Supervision
*Goal: Sequenced bootloader transitions, process wait_group supervision, and crash information collection.*

1. **Sysmgr Client/Server Porting (`target/earlgrey/services/sysmgr`)**:
   - Pull client/server directories from `hwe2`.
   - Replace standard `pw_log` occurrences inside the boot sequencer with generic `util_zfmt` logger macros.
2. **Supervisor Thread (`target/earlgrey/firmware/hwe/sysmgr.rs`)**:
   - Map the supervisor `WaitGroup` listening for `Signals::JOINABLE` on `flash_server`, `usbdfu`, and `platform` handles.
   - Port Retention RAM (`target/earlgrey/util/ret_ram.rs`). On unexpected task exit, record the termination status to retention RAM and emit a structured log event before triggering a reboot. Full crash recovery policy is deferred to a later milestone.

### Phase 4: Platform Tasks
*Goal: Mappings for pinmux registers, GPIO inputs/outputs, and physical board multiplexers.*

1. **Port Platform Daemon (`target/earlgrey/firmware/hwe/platform.rs`)**:
   - Extract from `hwe2` and clean up references.
   - Set up pin definitions and define the task's `UTIL_ZFMT_LOGGER` static.

### Phase 5: Final System Image & Verification
*Goal: Compile full system and run multi-sim sweeps.*

1. **HWE System Configuration (`target/earlgrey/firmware/hwe/system.json5`)**:
   - Map Flash/RAM boundaries and configure logging channels.
2. **Simulations Sweep**:
   - Execute Verilator (`hwe_verilator_test`) and FPGA Board Tests (`hwe_hyper310_test`, `hwe_hyper340_test`).

---

## 5. API Proposals & Migration Models

This section details the exact implementation of the Earlgrey-specific timer utility, the generic `util/zfmt` library, the link-symbol definitions, and task entrypoints.

### Earlgrey Timer Utility (`target/earlgrey/util/timer.rs`)

A clean wrapper providing clock access to the platform.

```rust
// target/earlgrey/util/timer.rs

#![no_std]

use registers::rv_timer::RvTimer;

pub struct EarlGreyTimer {
    device: RvTimer,
}

impl EarlGreyTimer {
    /// # Safety
    /// Must ensure exclusive access to RvTimer register mapping.
    pub const unsafe fn new() -> Self {
        Self {
            device: unsafe { RvTimer::new() },
        }
    }

    /// Returns the current standard 64-bit hardware tick count.
    pub fn read_ticks(&self) -> u64 {
        let regs = self.device.regs();
        loop {
            let hi1 = regs.timer_v_upper0().read();
            let low = regs.timer_v_lower0().read();
            let hi2 = regs.timer_v_upper0().read();
            if hi1 == hi2 {
                return ((hi1 as u64) << 32) | (low as u64);
            }
        }
    }
}
```

### Generic project-wide Static Link Logger (`util/zfmt`)

This generic crate declares the `extern "Rust"` static, exposes a safe `logger()` accessor, and provides logging macros. Clock access is resolved at build time via Bazel `select`.

```rust
// util/zfmt/lib.rs

#![no_std]

use zfmt::{FlatSend, FlatAdapter, ZfmtU64};
use userspace::syscall;
use userspace::time::Instant;

// Clock implementation is selected at build time via Bazel select in BUILD.bazel:
//   earlgrey target  → src/clock_earlgrey.rs  (wraps EarlGreyTimer::read_ticks)
//   host / default   → src/clock_host.rs      (returns 0)
mod clock;

// Tentative buffer size. All log events must fit within this limit.
// Revisit once the full event vocabulary is characterized.
pub const LOG_BUFFER_SIZE: usize = 256;

/// Generic logger. Immutable; holds only the task's IPC handle.
pub struct IpcLogger {
    handle: u32,
}

impl IpcLogger {
    /// Sentinel value meaning "logging is disabled for this task."
    /// A task may define UTIL_ZFMT_LOGGER with this handle to suppress all log output.
    pub const DISABLED: u32 = u32::MAX;

    pub const fn new(handle: u32) -> Self {
        Self { handle }
    }
}

impl FlatSend for IpcLogger {
    fn timestamp(&self) -> ZfmtU64 {
        ZfmtU64::from_u64(clock::now_ticks())
    }

    fn send(&self, data: &[u8]) {
        if self.handle != Self::DISABLED {
            let mut rx_dummy = [0u8; 0];
            let _ = syscall::channel_transact(
                self.handle,
                data,
                &mut rx_dummy,
                Instant::MAX
            );
        }
    }
}

// --- Link-Time Symbol Resolution ---

extern "Rust" {
    /// Resolved at link-time to the active task's #[no_mangle] static UTIL_ZFMT_LOGGER.
    static UTIL_ZFMT_LOGGER: FlatAdapter<IpcLogger, LOG_BUFFER_SIZE>;
}

/// Safe accessor; confines the single `unsafe` deref here rather than at every call site.
#[inline(always)]
pub fn logger() -> &'static FlatAdapter<IpcLogger, LOG_BUFFER_SIZE> {
    unsafe { &UTIL_ZFMT_LOGGER }
}

// --- Global Logging Macros ---

#[macro_export]
macro_rules! log_info {
    ($event:expr) => {
        $crate::zfmt::log_info!($crate::logger(), $event);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($event:expr) => {
        $crate::zfmt::log_warn!($crate::logger(), $event);
    };
}

#[macro_export]
macro_rules! log_error {
    ($event:expr) => {
        $crate::zfmt::log_error!($crate::logger(), $event);
    };
}
```

### USB Task Log Ring Buffer (`//target/earlgrey/services/usbdfu`)

The ring buffer and sequence stamping live in the service library. The drain modules and event loop sketch show how the thin entry point in `//target/earlgrey/firmware/hwe/usbdfu.rs` assembles and drives them.

```rust
// target/earlgrey/services/usbdfu/log_ring.rs

/// Initial size. Revisit after DFU page-write latency is characterized.
const LOG_RING_SIZE: usize = 4096;

struct LogRingBuffer {
    buf: [u8; LOG_RING_SIZE],
    write: usize,
    #[cfg(feature = "log_uart")]
    read_uart: usize,
    #[cfg(feature = "log_cdc_acm")]
    read_cdc: usize,
}

impl LogRingBuffer {
    /// Write a complete frame. Evicts oldest complete frame(s) for any cursor
    /// that would otherwise be lapped. Caller has already stamped the sequence number.
    fn write_frame(&mut self, frame: &[u8]) { ... }

    /// Bytes available to read from `cursor` before it reaches the write position.
    fn readable_from(&self, cursor: usize) -> usize { ... }
}

// Global sequence counter. Incremented once per received IPC log frame.
static mut LOG_SEQ: u32 = 0;

fn receive_log_frame(ring: &mut LogRingBuffer, raw: &[u8]) {
    let seq = unsafe {
        LOG_SEQ = LOG_SEQ.wrapping_add(1);
        LOG_SEQ
    };
    // Copy raw frame, stamp sequence number into EventHeader, write to ring.
    let mut frame = [0u8; LOG_BUFFER_SIZE];
    let len = raw.len().min(LOG_BUFFER_SIZE);
    frame[..len].copy_from_slice(&raw[..len]);
    stamp_sequence(&mut frame[..len], seq);
    ring.write_frame(&frame[..len]);
}
```

```rust
// target/earlgrey/services/usbdfu/uart_output.rs  (feature = "log_uart")

/// Non-blocking drain: copies from the UART read cursor into the UART TX FIFO
/// until the FIFO is full or no more buffered data remains.
pub fn drain(ring: &mut LogRingBuffer, uart: &Uart) {
    while ring.readable_from(ring.read_uart) > 0 && uart.tx_fifo_has_space() {
        let byte = ring.read_byte(&mut ring.read_uart);
        uart.write_byte(byte);
    }
}
```

```rust
// target/earlgrey/services/usbdfu/cdc_output.rs  (feature = "log_cdc_acm")

/// Non-blocking drain: submits one bulk packet to the CDC-ACM IN endpoint
/// if an endpoint buffer is available and there is buffered data to send.
pub fn drain(ring: &mut LogRingBuffer, cdc: &CdcAcm) {
    if !cdc.in_endpoint_ready() { return; }
    let n = ring.readable_from(ring.read_cdc).min(CDC_PACKET_SIZE);
    if n == 0 { return; }
    let mut packet = [0u8; CDC_PACKET_SIZE];
    ring.read_bytes(&mut ring.read_cdc, &mut packet[..n]);
    cdc.submit_packet(&packet[..n]);
}
```

```rust
// Event loop structure in target/earlgrey/firmware/hwe/usbdfu.rs (thin entry point)

loop {
    if ipc_log_readable() {
        let len = syscall::channel_read(handle::LOGGING_CHANNEL, 0, &mut ipc_buf);
        receive_log_frame(&mut ring, &ipc_buf[..len]);
    }

    handle_dfu_events(&mut dfu_state);

    #[cfg(feature = "log_uart")]
    uart_output::drain(&mut ring, &uart);
    #[cfg(feature = "log_cdc_acm")]
    cdc_output::drain(&mut ring, &cdc);

    syscall::object_wait_many(&handles, deadline);
}

// Inside the DFU flash-write loop:
for page in pages.iter() {
    flash_service.write_page(page)?;
    #[cfg(feature = "log_uart")]
    uart_output::drain(&mut ring, &uart);
    #[cfg(feature = "log_cdc_acm")]
    cdc_output::drain(&mut ring, &cdc);
}
```

### Task Entrypoint and Symbol Export

Every userspace task binary defines one `#[no_mangle]` static. That is the entirety of the per-task logging boilerplate:

```rust
// target/earlgrey/firmware/hwe/sysmgr.rs (Example Process Startup)

#![no_std]
#![no_main]

use sysmgr_codegen::handle;
use pw_status::Result;
use util_zfmt::{FlatAdapter, IpcLogger, LOG_BUFFER_SIZE};

// LOG_BUFFER_SIZE bytes of .bss; zero flash cost; resolved by util_zfmt's extern "Rust" static.
#[no_mangle]
static UTIL_ZFMT_LOGGER: FlatAdapter<IpcLogger, LOG_BUFFER_SIZE> =
    FlatAdapter::new(IpcLogger::new(handle::LOG_IPC_HANDLER));

#[derive(zfmt::Zfmt)]
#[zfmt(format = "System Manager process started")]
struct SysmgrStartEvent;

#[userspace::entry]
fn entry() -> Result<()> {
    util_zfmt::log_info!(SysmgrStartEvent);

    // ... Run service loop
    Ok(())
}
```

---

*Co-authored by Jetski, pairing with a Google DeepMind researcher.*
