//! We'll write an os running on riscv64gc-unknown-none-elf,
//! so we need to add [no_std crate level attribute](https://docs.rust-embedded.org/book/intro/no-std.html).
//! Then the `fn main()` requires std to initialize, because we disabled std,
//! We need to add `no_main` attribute to disable initialize with std.
//!
//! After add `no_std` and `no_main`, the program contains no logic provided by std, we will re-write ourself.

#![no_std]
#![no_main]
#![warn(missing_docs)]

mod lang_items;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

/// With attribute `no_mangle` to avoid rustc mangle the function name, which leads to a link failure.
/// In the opening state, we need to alloc stack frame and save the function call context, which is
/// the lowest stack frame.
///
/// In the core initializing, we need to complete clear for `.bss` section, before we use any
/// global variable allocted to `.bss`, we need to ensure it is cleared, we do this work in the
/// beginning of [`rust_main()`] with [`clear_bss()`] after the control is transferred to rust, so
/// we can no longer write assembly language to deal with this!
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    loop {}
}

/// Clear `.bss` section.
/// We will find the global variable `sbss` and `ebss` from the `linker.ld` which point the beginning
/// and the end addr to be clear. So we just traverse this area and make each byte to zero.
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}
