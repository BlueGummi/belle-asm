.ssp $1
.sbp $50
.start $100
    int #11
    jz @time_waster
time_waster:
    pop %r0 ; so the stack doesn't overflow
    nop
    jz @time_waster
