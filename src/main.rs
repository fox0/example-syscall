// #![no_std]
// #![no_main]
#![warn(clippy::pedantic)]

mod syscall;

use crate::syscall::arch::{sys_exit, sys_write, STDOUT};

fn main() {
    let _ = sys_write(STDOUT, "Hello from asm!\n");
    // println!("write returned: {}", ret);
    sys_exit(0);
}
