//! Test access S-CSR in U mode.

#![no_std]
#![no_main]

extern crate user_lib;

use log::warn;
use riscv::register::sstatus::{self, SPP};

#[no_mangle]
fn main() -> i32 {
    warn!("Try to access privileged CSR in U mode, kernel should kill this application!");
    unsafe { sstatus::set_spp(SPP::User) }
    0
}
