	; $1 is x
	; $2 is y
	mov %r0, #0 ; x
	mov %r1, #1
	st $1, %r0
	st $2, %r1
	mov %r2, #0
	set #1
	jz @loop
loop:
	mov %r2, #0
	ld %r0, $1
	ld %r1, $2
	add %r2, %r1
	add %r2, %r0
	mov %r0, %r1
	mov %r1, %r2
	st $1, %r0
	st $2, %r1
	set #1
	jz @loop

