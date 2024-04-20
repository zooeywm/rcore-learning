//! App management syscalls
use log::trace;

use crate::batch::run_next_app;

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("Application exited with code {}", exit_code);
    run_next_app()
}
