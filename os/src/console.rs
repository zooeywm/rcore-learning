//! Console helper.

use core::fmt::{Arguments, Write};

use crate::sbi::console_putchar;

struct Stdout;

impl Write for Stdout {
    /// Define the way our os write str with sbi_rt.
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
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
