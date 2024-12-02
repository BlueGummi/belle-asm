				
				
.sbp $100
.ssp $1
.start $500
				
				
				
						push %r0
					    	push %r1
						int #11
						jz @loop
loop:
				
				
						pop %r0
				
				
				
				
				
						push %r1
						int #2
						int #11
						jz @loop
					
