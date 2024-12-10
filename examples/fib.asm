    ; This program calculates the fibonacci sequence to the largest signed 16 bit integer value
    ; $1 is x
    ; $2 is y
.sbp $10
.ssp $1
.start $100
    mov %r0, #0         ; first value of fibonacci
    mov %r1, #1         ; second value of fibonacci
    push %r0            ; r0 onto stack
    push %r1            ; r1 onto stack 
    int #11             
    jz @fib_loop
fib_loop:
    pop %r4             ; pop the jump address and save it
    mov %r2, #0         ; final value of fibonacci
    pop %r1             ; load values back into registers
    pop %r0
    add %r2, %r1        ; z += y
    add %r2, %r0        ; z += x
    mov %r0, %r1        ; x = y
    mov %r1, %r2        ; y = z
    push %r0            ; push values back onto stack
    push %r1
    jo @finish
    int #2              ; print the value in register 2
    jz @fib_loop        ; continue loop
finish:
    hlt
