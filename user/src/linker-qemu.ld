/* Same as os linkage, but except the BASE_ADDRESS */

OUTPUT_ARCH(riscv)
ENTRY(_start)

/* U-mode code start address */
BASE_ADDRESS = 0x80400000;

SECTIONS
{
    . = BASE_ADDRESS;
    stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    edata = .;
    sbss = .;
    .bss : {
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    ebss = .;
    /DISCARD/ : {
        *(.eh_frame)
        *(.debug*)
    }
}
