#![allow(dead_code)]
use std::arch::asm;

pub const STDOUT: usize = 1;

//todo enum
const SYS_WRITE: u64 = 1;
const SYS_EXIT: u64 = 60;

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn sys_write(fd: usize, buf: &str) -> usize {
    let ret = unsafe { syscall3(SYS_WRITE, fd as u64, buf.as_ptr() as u64, buf.len() as u64) };
    ret as usize
}

#[allow(clippy::empty_loop)]
pub fn sys_exit(error_code: u8) -> ! {
    let _ = unsafe { syscall1(SYS_EXIT, u64::from(error_code)) };
    loop {}
}

unsafe fn syscall1(name: u64, arg1: u64) -> u64 {
    let ret;
    asm!(
        "syscall",
        in("rax") name,
        in("rdi") arg1,
        lateout("rax") ret,
    );
    ret
}

unsafe fn syscall3(name: u64, arg1: u64, arg2: u64, arg3: u64) -> u64 {
    let ret;
    asm!(
        "syscall",
        in("rax") name,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        out("rcx") _, // clobbered by syscalls
        out("r11") _, // clobbered by syscalls
        lateout("rax") ret,
    );
    ret
}
