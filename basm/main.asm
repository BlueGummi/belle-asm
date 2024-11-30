    int
	set #3
    cl #3
    mul %r0, &r7
   mov %r1, &r3
   mov %r1, &$40
   add %r3, %r2 ; comment
   add %r4, #127
   ld %r2, $500
   st $500, %r2
   jz $399
   int #2
   add %r2, #-43
   mov %r4, #40
   hlt
   ret
   cmp %r3, %r2


   mov %r0, #0
  mov %r1, #1
   mov %r2, #10 ; limit
   mov %r3, %r0 ;r3 as counter
  int #4
	jz @fib
   fib:
     cmp %r3, %r2

	add %r4, %r1
	add %r4, %r0
	mov %r0, %r1
	mov %r4, %r0
	add %r3, #1
	jz @fib
ret
mov %r2, #4
