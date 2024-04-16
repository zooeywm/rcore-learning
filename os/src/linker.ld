OUTPUT_ARCH(riscv) /* Set target platform as riscv */
ENTRY(_start) /* Set entire entry of this program as _start defined in entry.asm */
BASE_ADDRESS = 0x80200000; /* To work on QEMU, we need to put the initialize code to this addr */

/* Codes blow reflects merging of sections in linking progress */
SECTIONS
{
    /* '.' refers to current addr, assign a value can adjust next section addr beginning */
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    . = ALIGN(4K);
    /* We can also assign some global symbol as '.' to record addr at this time */
    etext = .;
    srodata = .;
    /* .rodata before ':' refers to a section name in the final binary */
    .rodata : {
        /* Put sections of input object file by order into this section: .rodata */
        /* format: <ObjectFile>(SectionName) */
        /* Means section named SctionName from ObjectFile needs to be put in */
        /* We can use regex to refer to possible input ObjectFile and section name */
        /* Below means put sections by little-endian in the same naming section from all object files */
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
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}
