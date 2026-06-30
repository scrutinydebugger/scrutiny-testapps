OUTPUT_FORMAT("elf32-sparc")
OUTPUT_ARCH(sparc)
ENTRY(_start)

/*
 * leon3_generic memory map (QEMU defaults):
 *   0x00000000  Boot ROM  (16 MB)
 *   0x40000000  SDRAM     (64 MB)  <-- we run from here
 *   0x80000000  APB peripherals
 */
MEMORY {
    RAM (rwx) : ORIGIN = 0x40000000, LENGTH = 64M
}

SECTIONS {
    . = ORIGIN(RAM);

    .text : {
        *(.text .text.*)
        *(.rodata .rodata.*)
    } > RAM

    . = ALIGN(4);
    .data : {
        *(.data .data.*)
    } > RAM

    . = ALIGN(4);
    .bss : {
        _bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(4);
        _bss_end = .;
    } > RAM

    /* 8 KB stack growing downward from _stack_top */
    . = ALIGN(8);
    . += 8192;
    _stack_top = .;
}
