				
				
.sbp $100
.ssp $1
.start $500
<<<<<<< HEAD
	int #51  ; halt on overflow
	mov %r0, #0 ; x
	mov %r1, #1 ; y
	push %r0
    push %r1
	int #11
	jz @loop
=======
				
				
				
						push %r0
					    	push %r1
						int #11
						jz @loop
>>>>>>> 8ff5f48a34ec8fe5530eb48126ea28002ff95b58
loop:
				
				
						pop %r0
				
				
				
				
				
						push %r1
						int #2
						int #11
						jz @loop
					
