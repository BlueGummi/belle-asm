; This program calculates the fibonacci sequence to the largest signed 16 bit integer value
    ; $1 is x
	; $2 is y
.sbp $10
.ssp $1
.start $100
	int #51  ; halt on overflow
	mov %r0, #0 ; x
	mov %r1, #1 ; y
	push %r0 ; r0 onto stack
	push %r1 ; r1 onto stack 
	int #11 ; set zero flag
	jz $108 ; this becomes unconditional 
    pop %r4 ; pop the jump address and save it
	mov %r2, #0 ; z
	mov %r1, #0 ; prepare for addition
	mov %r0, #0
	pop %r1 ; load them back into registers
	pop %r0
	add %r2, %r1 ; z = z + y
	add %r2, %r0 ; z = z + x
	mov %r0, %r1 ; x = y
	mov %r1, %r2 ; y = z
	push %r0 ; push them back onto the stack
	push %r1
	int #2 ; print the value in register 2
	jz $108

