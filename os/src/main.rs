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
