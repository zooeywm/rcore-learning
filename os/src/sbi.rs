//! SBI calls by sbi_rt.

use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};

/// Call SBI putchar
pub fn console_putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

/// `failure` to represent whether the os is exit normally.
pub fn shutdown(failure: bool) -> ! {
    if failure {
        system_reset(Shutdown, SystemFailure);
    } else {
        system_reset(Shutdown, NoReason);
    }
    unreachable!()
}
