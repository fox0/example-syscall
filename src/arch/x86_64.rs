#![allow(dead_code)]
use std::arch::asm;

pub const STDOUT: usize = 1;

#[derive(Copy, Clone)]
enum Syscalls {
    Write = 1,
    Exit = 60,
}

#[cfg(target_pointer_width = "64")]
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn sys_write(fd: usize, buf: &str) -> usize {
    let ret = unsafe {
        syscall3(
            Syscalls::Write,
            fd as u64,
            buf.as_ptr() as u64,
            buf.len() as u64,
        )
    };
    ret as usize
}

#[allow(clippy::empty_loop)]
pub fn sys_exit(error_code: u8) -> ! {
    let _ = unsafe { syscall1(Syscalls::Exit, error_code) };
    loop {}
}

unsafe fn syscall1<T: Into<u64>>(name: Syscalls, arg1: T) -> u64 {
    let arg0 = name as u64;
    let arg1 = arg1.into();
    let ret;
    asm!(
        "syscall",
        in("rax") arg0,
        in("rdi") arg1,
        lateout("rax") ret,
    );
    ret
}

unsafe fn syscall3<T: Into<u64>>(name: Syscalls, arg1: T, arg2: T, arg3: T) -> u64 {
    let arg0 = name as u64;
    let arg1 = arg1.into();
    let arg2 = arg2.into();
    let arg3 = arg3.into();
    let ret;
    asm!(
        "syscall",
        in("rax") arg0,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        out("rcx") _, // clobbered by syscalls
        out("r11") _, // clobbered by syscalls
        lateout("rax") ret,
    );
    ret
}
