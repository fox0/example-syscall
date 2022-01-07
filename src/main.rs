use std::arch::asm;

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

fn main() {
    let buf = "Hello from asm!\n";
    let ret: i32;
    unsafe {
        asm!(
            "syscall",
            in("rax") 1, // syscall number
            in("rdi") 1, // fd (stdout)
            in("rsi") buf.as_ptr(),
            in("rdx") buf.len(),
            out("rcx") _, // clobbered by syscalls
            out("r11") _, // clobbered by syscalls
            lateout("rax") ret,
        );
    }
    // println!("write returned: {}", ret);
}
