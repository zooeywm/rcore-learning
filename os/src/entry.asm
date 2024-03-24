    .section .text.entry # Show that we want to push the entire content to a section named .text.entry.
    .globl _start # Tell the compilor that _start is a global symbol, can be used by other object file.
_start: # Declare a symbol named _start, means the address of _start is the address of li instruction.
    li x1, 100 # Load 100 Immediately to register x1.
