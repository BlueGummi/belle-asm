	; $1 is x
	; $2 is y
.start $500
	mov %r1, #0 ; x
	mov %r2, #1 ; y
	st $1, %r1
	st $2, %r2
	set #1
	jz @loop
loop:
	mov %r3, #0 ; z
	ld %r1, $1 ; load them back
	ld %r2, $2
	add %r3, %r2 ; z = z + y
	add %r3, %r1 ; z = z + x
	mov %r1, %r2 ; x = y
	mov %r2, %r3 ; y = z
	st $1, %r1 ; put them back
	st $2, %r2
	set #1
	jz @loop

