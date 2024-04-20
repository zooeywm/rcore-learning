//! Hello world application

#![no_std]
#![no_main]

/// To use panic handler.
extern crate user_lib;

use log::info;

#[no_mangle]
fn main() -> i32 {
    info!("Hello, world!");
    0
}
