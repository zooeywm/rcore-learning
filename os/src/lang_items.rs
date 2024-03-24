//! Items that running this os required.

use core::panic::PanicInfo;

/// When panic happened, we need to use `#[panic_handler]` to
/// specify [`panic_handler()`] as panic handler.
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
