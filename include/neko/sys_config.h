#ifndef _NEKO_SYS_CONFIG_H
#define _NEKO_SYS_CONFIG_H 1

#ifdef _TARGET_ISA

    #define _NEKO_ISA _TARGET_ISA

    #define _NEKO_ISA_X86_32      1
    #define _NEKO_ISA_X86_64      2
    #define _NEKO_ISA_RISCV_32   3
    #define _NEKO_ISA_RISCV_64   4

    #if (_NEKO_ISA == 0)
    #  error "_NEKO_ISA has incorrect value (0)"
    #endif

#else
#  error "<neko/sys_config.h> please define _TARGET_ISA"
#endif // _TARGET_ISA

#endif // _NEKO_SYS_CONFIG_H
