#[cfg(feature = "anti-debug")]
pub mod anti_debug;
#[cfg(feature = "anti-vm")]
pub mod anti_vm;
#[cfg(feature = "canary")]
pub mod canary;
#[cfg(feature = "checkpoint")]
pub mod checkpoint;
pub mod integrity;
pub mod self_check;
pub mod time_guard;
pub mod heuristic_ai;
pub mod anti_inject;
pub mod mem_integrity;
pub mod parent_process;
pub mod anti_suspend;
pub mod anti_dump;
pub mod hw_breakpoint;
pub mod secure_storage;
pub mod var_obfuscator;
pub mod decoy_honeypot;
pub mod chaotic_thread;
pub mod kinematic_anomaly;
pub mod mem_scan;

pub use integrity::{protect, ProtectionConfig};
pub use self_check::{init_self_check, verify_self_check};
pub use time_guard::{check_time_tampering, init_time_guard};
pub use mem_scan::scan_for_rwx_memory;
