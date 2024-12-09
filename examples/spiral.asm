    ; This program increments the number being printed to stdout, then detects an overflow, before decrementing it again
    ; It is indefinite
.start $303
    mov %r5, #31
    mov %r4, #7
    mul %r5, %r4 ; 31 x 7 is in r5
    mov %r4, #100
    add %r4, #51 ; 151
    mul %r5, %r4
    int #11
    jz @loop
loop:
    pop %r4 ; pop off return address
    add %r0, #1
    int #0
    cmp %r0, %r5
    int #13
    jz @loop
    int #11
    jz @sub_loop
sub_loop:
    pop %r4 ; pop off that return address
    add %r0, #-1
    int #0
    cmp %r0, %r1
    int #13
    jz @sub_loop
    int #11
    jz @loop
