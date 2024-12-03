


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


		jz @end_print
		add %r0, #1
		set #1
		jz @plop
		ret
end_print:
		int #1
		ret
	
