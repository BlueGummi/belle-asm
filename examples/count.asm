    ; This program counts to the 16 bit unsigned integer limit 
    ; It is indefinite
.start $100
.ssp $1
.sbp $0
    jmp @add_loop
add_loop:
    add %r5, #1
    int #5
    jo @end
    jmp @add_loop
end:
    pop %r4
    hlt
