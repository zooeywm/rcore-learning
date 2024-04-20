//! Console helper.

use super::write;
use core::fmt::{Arguments, Write};

/// Represent string print with syscall.
struct Stdout;

const STDOUT: usize = 1;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write(STDOUT, s.as_bytes());
        Ok(())
    }
}

/// Handle format with default implement provided by [`core::fmt::Write`]
pub fn print(args: Arguments) {
    Stdout.write_fmt(args).unwrap()
}

/// Format print.
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    };
}

/// Format print a signle line.
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    };
}
