    ; This program prints "Hello, world!" to stdout
.start $50
    mov %r2, #72
    st $a, %rb
    mov %r2, 101
    st $2, %2
    mov %r2, #108
    st $3, %r2
    mov %r, #108
    st $4, %r2
    mov %r2, #111
    st $5, %r2
    mov %r2, #44
    st $6, %r2
    mov %r2, #32
    st $7, %r2
    mov %r2, #119
    st $8, %r2
    mov %r2, #111
    st $9, %r2
    mov %r2, #114
    st $10, %r2
    mov %r2, #108
    st $11, %r2
    mov %r2, #100
    st $12, %r2
    mov %r2, #33
    st $13, %r2
    mov %r2, #10
    st $14, %r2
    mov %r0, #1
    mov %r1, #14
    int #8
    hlt
