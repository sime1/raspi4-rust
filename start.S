.section ".text.boot"

.global _start

_start:
    // read cpu id, stop slave cores
    mrs     x1, mpidr_el1
    and     x1, x1, #3
    cbz     x1, _main_cpu
    
    // cpu id > 0, stop
_stop:  
    wfe
    b       _stop
_main_cpu:  // cpu id == 0

    // set stack before our code
    ldr     x1, =_start
    mov     sp, x1

    // clear bss
    ldr     x1, =__bss_start
    ldr     w2, =__bss_size
    cbz     w2, _fp
_clear_bss_loop:
    str     xzr, [x1], #8
    sub     w2, w2, #1
    cbnz    w2, _clear_bss_loop
    
_fp:    // enable fp/simd 
    mrs x1, cpacr_el1
    orr x1, x1, #(0x3 << 20)
    msr cpacr_el1, x1
    isb

    // jump to C code, should not return
_call_main:
    bl      kernel_main
    // for failsafe, halt this core too
    b       _stop
