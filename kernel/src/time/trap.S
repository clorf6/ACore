    .section .text
    .globl __timetrap
    .align 2
__timetrap: 
    csrrw sp, mscratch, sp
    sd t0, 16(sp)
    sd t1, 24(sp)
    sd t2, 32(sp)
    ld t0, 0(sp)
    ld t1, 8(sp)
    ld t2, 0(t0)
    add t2, t2, t1
    sd t2, 0(t0)
    li t0, 2
    csrw sip, t0
    ld t0, 16(sp)
    ld t1, 24(sp)
    ld t2, 32(sp)
    csrrw sp, mscratch, sp
    mret