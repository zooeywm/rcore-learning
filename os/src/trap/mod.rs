//! Trap handling

use core::arch::global_asm;

use log::error;
use riscv::register::{
    scause::{self, Exception::*, Trap},
    stval, stvec,
    utvec::TrapMode,
};

use crate::{batch::run_next_app, syscall::syscall};

pub use self::context::TrapContext;

mod context;

global_asm!(include_str!("trap.S"));

/// Init trap with set stvec to Direct mode
pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe { stvec::write(__alltraps as usize, TrapMode::Direct) }
}

#[no_mangle]
/// Handle an interrupt, exception, or system call from user space
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(UserEnvCall) => {
            cx.sepc += 4;
            // a7 - syscall ID, a0~a2: args, a0: also record return value
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(e) => {
            error!("{e:?} in application, kernel killed it.");
            run_next_app();
        }
        _ => {
            panic!(
                "Unsupported trap {:#?}, stval = {:#x}!",
                scause.cause(),
                stval
            )
        }
    }
    cx
}
