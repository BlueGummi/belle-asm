    .ascii "pow pow pow"
    push #50
pop %r7
cl #4


.start $500
   mov %r0, #4
    mul %r4, #3
   st $500, %r4
   mov %r0, #100
   mul %r0, #5
   mov %r1, &r0
   add %r3, %r2 ; comment
   add %r4, #127
   ld %r2, $500
   st $500, %r2
   jnz $399
   int #2
   mov %r4, #40
   hlt
   ret
   cmp %r3, %r2


   mov %r0, #0
  mov %r1, #1
   mov %r2, #10 ; limit
   mov %r3, %r0 ;r3 as counter
  int #4
   call @fib
   fib:
     cmp %r3, %r2

	add %r4, %r1
	add %r4, %r0
	mov %r0, %r1
	mov %r4, %r0
	add %r3, #1
	call @fib
ret
mov %r2, #4