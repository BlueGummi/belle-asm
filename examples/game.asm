.start [100]
.ssp [99]
.sbp [99]
	mov r1, '-' ; hyphen ascii
	push 2
begin:
	pop r5 ; avoiding overflow
	st &r3, r1 ; put it into memory
	add r3, 1 ; accumulator
	cmp r3, 30
	jz @next
	jmp @begin
next:	
	mov r3, 10
	st [30], r3
	mov r0, 0
	mov r1, 30
	int 8
	int 12
	int 9
	cmp r0, 68
	cmp r0, 100
	jz @move_right
	cmp r0, 65
	cmp r0, 97
	jz @move_left
	jmp @done
move_right:
	pop r5
	mov r1, 45
	add r4, 1
	ret
move_left:
	pop r5
	mov r1, 45
	add r4, 1
	ret
done:
	hlt
