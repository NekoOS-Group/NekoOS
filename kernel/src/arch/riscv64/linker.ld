OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0xffffffffc0200000;

SECTIONS{

    . = BASE_ADDRESS;
    skernel = .;

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
            *(.bss.stack)
            *(.bss .bss.*)
            *(.sbss .sbss.*)
            *(.bss.heap)
        }
        . = ALIGN(4K);
        ebss = .;

    ekernel = .;


    /DISCARD/ : {
        *(.eh_frame)
    }
}