// // #![feature(asm)]
// use std::arch::asm;
//
// pub unsafe fn exit(error_code: k_int) {
//     call!(kty::__NR_exit, error_code);
// }
//
// // #[cfg(target_pointer_width = "64")]
// // #[path = "x86_64/x64.rs"]
// // mod abi;
//
// // mod asm {
//
// // use crate::syscall::raw::arch::abi::SCT;
//
// #[inline(always)]
// pub unsafe fn syscall0(n: SCT) -> SCT {
//     let ret: SCT;
//     asm!("syscall" : "={rax}"(ret)
//                    : "{rax}"(n)
//                    : "rcx", "r11", "memory"
//                    : "volatile");
//     ret
// }
//
// // #[inline(always)]
// // pub unsafe fn syscall1(n: SCT, a1: SCT) -> SCT {
// //     let ret: SCT;
// //     asm!("syscall" : "={rax}"(ret)
// //                    : "{rax}"(n), "{rdi}"(a1)
// //                    : "rcx", "r11", "memory"
// //                    : "volatile");
// //     ret
// // }
// //
// // #[inline(always)]
// // pub unsafe fn syscall2(n: SCT, a1: SCT, a2: SCT) -> SCT {
// //     let ret: SCT;
// //     asm!("syscall" : "={rax}"(ret)
// //                    : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2)
// //                    : "rcx", "r11", "memory"
// //                    : "volatile");
// //     ret
// // }
// //
// // #[inline(always)]
// // pub unsafe fn syscall3(n: SCT, a1: SCT, a2: SCT, a3: SCT) -> SCT {
// //     let ret: SCT;
// //     asm!("syscall" : "={rax}"(ret)
// //                    : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3)
// //                    : "rcx", "r11", "memory"
// //                    : "volatile");
// //     ret
// // }
// //
// // #[inline(always)]
// // pub unsafe fn syscall4(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT) -> SCT {
// //     let ret: SCT;
// //     asm!("syscall" : "={rax}"(ret)
// //                    : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
// //                      "{r10}"(a4)
// //                    : "rcx", "r11", "memory"
// //                    : "volatile");
// //     ret
// // }
// //
// // #[inline(always)]
// // pub unsafe fn syscall5(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT, a5: SCT) -> SCT {
// //     let ret: SCT;
// //     asm!("syscall" : "={rax}"(ret)
// //                    : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
// //                      "{r10}"(a4), "{r8}"(a5)
// //                    : "rcx", "r11", "memory"
// //                    : "volatile");
// //     ret
// // }
// //
// // #[inline(always)]
// // pub unsafe fn syscall6(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT, a5: SCT, a6: SCT) -> SCT {
// //     let ret: SCT;
// //     asm!("syscall" : "={rax}"(ret)
// //                    : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
// //                      "{r10}"(a4), "{r8}"(a5), "{r9}"(a6)
// //                    : "rcx", "r11", "memory"
// //                    : "volatile");
// //     ret
// // }
// // }
