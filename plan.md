# TODOs for hardware enablement (HWE) firmware

## Top-level HWE features

1. Logging via zfmt logging
2. USB task to manage the usb peripheral and protocols
   a) CDC-ACM peripheral for logging.
   b) UART peripheral (simultaneous output of the log stream)
   c) Feature gate to enable/disable (b)
   d) Fused cdc-acm and uart input to implement a command processor in debug
      firmware.  Feature gated so it can be excluded by default.
   d) USB-DFU protocol to manage firmware updates
3. Flash service to manage access to flash.
4. System manager service to handle bootup, boot service and reset events.
   a) Future enhanement: observe and log panics from other tasks
5. Platform Integration task
   a) Owns pinmux/gpio peripheral
   b) Has GPIO pin mapping
   c) Manages platform reset and spi muxes.

## Directory structure

- //target/earlgrey/fimware/hwe:
  - Location of the firmware build: BUILD.bazel rules, system.json5, main
    entrypoints for each task.
- //target/earlgrey/services:
  - Location of earlgrey-specific services (e.g. sysmgr)
- //target/earlgrey/util:
  - Location of earlgrey-specific utility libraries: boot_log, boot_svc,
    ret_ram, rom_error, multi-bit bool definitions, etc.
- //target/util:
  - Location of project-wide utility libraries: precise errors, ipc helpers, io
    utilities, high-level types.
- //target/service:
  - Location of generic services: flash client & server libraries.
- //protocol/usb:
  - Location of usb stack and protocol implementations.
- //hal/blocking:
  - Location of project-wide hardware abstraction layers


