.ssp $1
.sbp $50
.start $100
    mov %r2, #87 ; w
    st $51, %r2
    mov %r2, #97 ; a
    st $52, %r2
    mov %r2, #115 ; s
    st $53, %r2
    mov %r2, #116 ; t
    st $54, %r2
    mov %r2, #101 ; e
    st $55, %r2
    mov %r2, #100 ; d
    st $56, %r2
    mov %r2, #32 ; 
    st $57, %r2
    mov %r2, #116 ; t
    st $58, %r2
    mov %r2, #105 ; i
    st $59, %r2
    mov %r2, #109 ; m
    st $60, %r2
    mov %r2, #101 ; e
    st $61, %r2
    mov %r2, #46 ; .
    st $62, %r2
    mov %r0, #51
    mov %r1, #62    
    int #11
    jz @time_waster
time_waster:
    pop %r4 ; so the stack doesn't overflow
    nop
    jz @print
    jz @time_waster
print:
    int #8
    ret
