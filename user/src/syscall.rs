//! Define [`syscall()`] on RISC-V, and other functions using it.

use core::arch::asm;

/// Write syscall ID
const SYSCALL_WRITE: usize = 64;
/// Exit syscall ID
const SYSCALL_EXIT: usize = 93;

/// Use `ecall` to generate `Environment call from U-mode Exception`, trap into S-mode.
/// Which also calls `ABI` or `syscall`.
/// Follow `RISC-V`'s ABI format, store arguments in corresponding registers,
/// then execute `ecall` to trigger Trap. After Trap, back to U-mode, will execute the next
/// instruction after `ecall`, and we can read return value from corresponding registers.
/// `x10~x17` represent for `a0~a7`
///
/// `x1` represent for `ra`
///
/// - `a0~a2` pass the arguments to S-mode, and `a0` saves the return from syscall
/// - `a7` saves syscall ID
///
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        // Use `asm!()` to insert assembly code with function context.
        asm!(
            "ecall",
            // Bind `args[0]` to `a0`, and bind returns value to `ret`
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

/// `Function` - Write buffer in memory into file
/// `Arguments`:
///     - `fd` - Fd to write
///     - `buf` - Start address of buffer
///     - `len` - Length of buffer
/// `Return`: Successfully wrote length
/// `syscall ID`: 64
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

/// `Function` - Exit application and tell the batch os
/// `Arguments`:
///     - `exit_code` - Application's exit code
/// `Return`: Never return
/// `syscall ID`: 93
pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}
