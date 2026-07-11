/*
 * Minimal SPARC V8 / Leon3 bare-metal startup.
 *
 * QEMU loads the ELF segments into RAM and sets PC = entry point (_start).
 * At that point the CPU is in supervisor mode with traps disabled (ET=0).
 * We set up the stack, zero BSS, then call main.
 */

    .section .text
    .global _start

_start:
    /* Point stack register to top of stack area defined in linker.ld.
     * SPARC stack grows downward; callees allocate frames via 'save'. */
    set     _stack_top, %sp

    /* Zero the BSS section */
    set     _bss_start, %l0
    set     _bss_end,   %l1
    cmp     %l0, %l1
    bge     2f
    nop
1:
    st      %g0, [%l0]
    add     %l0, 4, %l0
    cmp     %l0, %l1
    bl      1b
    nop
2:
    /* Enable FPU: set PSR.EF (bit 12).
     * Without this the CPU raises fp_disabled (trap 0x04) on any FP instruction. */
    mov     %psr, %g1
    sethi   %hi(0x1000), %g2
    or      %g1, %g2, %g1
    mov     %g1, %psr
    nop; nop; nop               /* 3 NOPs required after PSR write */

    call    main
    nop

    /* main returned (or never returns) — spin forever */
3:
    b       3b
    nop
