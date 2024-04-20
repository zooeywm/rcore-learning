/* Same as os linkage, but except the BASE_ADDRESS */

OUTPUT_ARCH(riscv)
ENTRY(_start)

/* U-mode code start address */
BASE_ADDRESS = 0x80400000;

SECTIONS
{
    stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    . = ALIGN(4K);
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(4K);
    edata = .;
    sbss = .;
    .bss : {
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    ebss = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}
