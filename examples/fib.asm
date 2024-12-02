	; $1 is x
	; $2 is y
.sbp $100
.ssp $1
.start $500
	int #15  ; halt on overflow
	mov %r0, #0 ; x
	mov %r1, #1 ; y
	push %r0
    push %r1
	int #11
	jz @loop
loop:
	mov %r2, #0 ; z
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
	jz @loop

