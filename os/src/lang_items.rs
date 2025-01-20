//! Items that running without std required.

use core::panic::PanicInfo;

use crate::{println, sbi::shutdown};

/// When panic happened, we need to use `#[panic_handler]` to
/// specify [`panic_handler`] as panic handler.
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    let err = info.message();
    if let Some(location) = info.location() {
        println!("Paniced at {}:{} {}", location.file(), location.line(), err);
    } else {
        println!("Panicked: {}", err);
    }
    // If OS is panic, shutdown the computer.
    shutdown(true)
}
