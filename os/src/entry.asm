    .section .text.entry # Show that we want to push the entire content to a section named .text.entry.
    .globl _start # Tell the compiler that _start is a global symbol, can be used by other object file.
_start: # Declare a symbol named _start, means the address of _start is the address of la instruction.
    la sp, boot_stack_top # Before the control is transferred to Rust entry point, set the stack pointer to the top of stack.
    call rust_main

    .section .bss.stack # We use bss.stack as stack area. Note that on RISCV, the stack addr is increse from higher to lower.
    .globl boot_stack_lower_bound # Define global stack bottom addr
boot_stack_lower_bound:
    .space 4096 * 16 # The max addr size is 64KiB
    .globl boot_stack_top # Define global stack top addr
boot_stack_top:
