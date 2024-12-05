; This program calculates the fibonacci sequence to the largest signed 16 bit integer value
    ; $1 is x
	; $2 is y
.sbp $10
.ssp $1
.start $100
	int #51  ; halt on overflow
	mov %r0, #0 ; x
	mov %r1, #1 ; y
	push %r0 ;r0 onto stack
	push %r1
	int #11
	jz $108
    pop %r4 ; pop jump
	mov %r2, #0 ; z
	mov %r1, #0
	mov %r0, #0
	pop %r1 ; load them back
	pop %r0
	add %r2, %r1 ; z = z + y
	add %r2, %r0 ; z = z + x
	mov %r0, %r1 ; x = y
	mov %r1, %r2 ; y = z
	push %r0 ; put them back
	push %r1
	int #2
	int #11
	jz $108

