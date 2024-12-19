ASM_FILES := $(wildcard *.asm)
EXECUTABLES := $(ASM_FILES:.asm=)

BASM := $(shell command -v basm 2>/dev/null)

ifneq ($(BASM),)
	BASM_CMD := basm
else
	$(error "basm not found")
endif

ifeq ($(OS),Windows_NT)
    RM := del /Q
else
    RM := rm -f
endif

all: clean $(EXECUTABLES)

%: %.asm
	$(BASM_CMD) -o $@ $<

clean:
	$(RM) $(EXECUTABLES) || true

.PHONY: all clean
