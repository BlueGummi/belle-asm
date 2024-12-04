.start $1
int #11
jz @noop
noop:
	nop
	jz @noop
