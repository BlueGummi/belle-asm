# BELLE - The complete program utility set for the Big Endian, Low Level Emulator

## Quickstart

Cargo, RustC, GCC, and Makefile compatibility must be present on the system if these programs are to be installed.

Every executable in this repository can have the --help flag passed to display helpful information.


```
./build.sh -w && ./install.sh -c # This installs the binaries to ~/.local/bin
```

Or, for Windows

```pwsh
.\build.ps1 -w && .\install.ps1 -c
```

The binaries can be run by calling `basm`, `belle`, or `bdump`.

### BELLE and the BELLE utilities in action:
![BELLE Usage GIF](https://github.com/BlueGummi/belle/blob/master/media/belle-usage.gif)

## Naming

**BELLE** is the *emulator*, whilst **BELLE-ISA/ISABELLE** is the *instruction set*.
### Note:

Subroutines do not resolve to addresses at compile time. Instead, subroutines are loaded into their own separate location in memory and jump instructions push a jump location onto a CPU field that is inaccessible through assembly.
This is because the 16 bit fixed length instruction is not large enough to account for subroutine address resolution and because the CPU stack is optional, so the CPU can be run without setting stack/base pointers.
