#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64.rs"]
pub mod arch;

#[cfg(target_arch = "x86")]
#[path = "arch/x86.rs"]
pub mod arch;

#[cfg(target_arch = "arm")]
#[path = "arch/arm.rs"]
pub mod arch;

#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64.rs"]
pub mod arch;
