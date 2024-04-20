//! Items that running without std required.

use core::panic::PanicInfo;

use log::error;

/// When panic happened, we need to use `#[panic_handler]` to
/// specify [`panic_handler`] as panic handler.
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    let err = info.message().unwrap();
    if let Some(location) = info.location() {
        error!("Paniced at {}:{} {}", location.file(), location.line(), err);
    } else {
        error!("Panicked: {}", err);
    }
    // When application panic, pend and wait for os instruction.
    loop {}
}
