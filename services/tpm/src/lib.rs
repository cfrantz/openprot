// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0
#![no_std]

pub mod platform;

// Re-export nullcrypto to ensure its symbols are linked.
pub use nullcrypto;
