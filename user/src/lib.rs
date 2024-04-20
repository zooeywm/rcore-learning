//! User lib

#![feature(linkage)]
#![no_std]
#![warn(missing_docs)]
#![feature(panic_info_message)]

use log::debug;
use syscall::{sys_exit, sys_write};

pub mod console;
pub mod lang_items;
mod logger;
mod syscall;

/// User batch task start
#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    logger::init();
    let SectionInfo {
        stext,
        etext,
        sdata,
        edata,
        srodata,
        erodata,
        sbss,
        ebss,
    } = get_sections();
    debug!(".text [{:#x}, {:#x})", stext, etext);
    debug!(".data [{:#x}, {:#x})", sdata, edata);
    debug!(".rodata [{:#x}, {:#x})", srodata, erodata);
    debug!(".bss [{:#x}, {:#x})", sbss, ebss);
    exit(main());
    unreachable!("unreachable after sys_exit!");
}

/// Weak linkage, to make it pass compile when bin lack of main function.
/// But will panic at runtime.
#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

/// Clear `.bss` section.
/// We will find the global variable `sbss` and `ebss` from the `linker-qemu.ld` which point the beginning
/// and the end addr to be clear. So we just traverse this area and make each byte to zero.
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}

/// Use write syscall
pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

/// Use exit syscall
pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

/// Record section start and end addr.
struct SectionInfo {
    stext: usize,
    etext: usize,
    sdata: usize,
    edata: usize,
    srodata: usize,
    erodata: usize,
    sbss: usize,
    ebss: usize,
}

/// Get [`SectionInfo`]
fn get_sections() -> SectionInfo {
    extern "C" {
        fn stext();
        fn etext();
        fn sdata();
        fn edata();
        fn srodata();
        fn erodata();
        fn sbss();
        fn ebss();
    }

    SectionInfo {
        stext: stext as usize,
        etext: etext as usize,
        sdata: sdata as usize,
        edata: edata as usize,
        srodata: srodata as usize,
        erodata: erodata as usize,
        sbss: sbss as usize,
        ebss: ebss as usize,
    }
}
