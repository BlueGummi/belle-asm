.start $300
		mov %r5, #31
		mov %r4, #7

		mov %r4, #100

		mul %r5, %r4
		set #1
		jz @loop
loop:
		add %r0, #-1
		int #0
		cmp %r0, %r5
		int #11
		jz @loop
		set #1
		jz @sub_loop
		ret
sub_loop:
		add %r0, #-1
		int #0
		cmp %r0, %r1
		int #11
		jz @sub_loop
		set #1
		jz @loop
		ret
