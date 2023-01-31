    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top
    call start

    .section .bss.stack
    .globl boot_stack
boot_stack:
    .space 65536
    .globl boot_stack_top
boot_stack_top: