; This program wastes time, and goes into an infinite loop of NOP instructions (No-Operation, does nothing)
.start $1
int #11
jz @noop
noop:
	nop
	jz @noop
