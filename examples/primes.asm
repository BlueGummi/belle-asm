.start $50 ; 50 spots more than enough
	; r1 will contain the number we are checking
	; r0 will be the number we are dividing by
	set #1
	mov %r1, #2
	int #1
	jz @prime_check_loop
prime_check_loop:
	add %r1, #1
	set #1
	mov %r0, #2
	jz @plop
	set #1
	jz @prime_check_loop
	ret
plop:
	mov %r2, %r1
	div %r2, %r0
	int #14 ; zflag = rflag
	int #11 ; flip it
	jz @end_print
	add %r0, #1
	set #1
	jz @plop
	ret
end_print:
	int #1
	ret

