# BELLE - The complete program utility set for the Big Endian, Low Level Emulator
[![Rust Build check](https://github.com/BlueGummi/belle/actions/workflows/ci.yml/badge.svg)](https://github.com/BlueGummi/belle/actions/workflows/ci.yml) [![Spellcheck](https://github.com/BlueGummi/belle/actions/workflows/spellcheck.yml/badge.svg)](https://github.com/BlueGummi/belle/actions/workflows/spellcheck.yml) [![Deploy Astro site to Pages](https://github.com/BlueGummi/belle/actions/workflows/publish.yml/badge.svg)](https://github.com/BlueGummi/belle/actions/workflows/publish.yml) 
All documentation is available on [the website for this project](https://bluegummi.github.io/belle) [![Deploy Astro site to Pages](https://github.com/BlueGummi/belle/actions/workflows/publish.yml/badge.svg)](https://github.com/BlueGummi/belle/actions/workflows/publish.yml)

## Quickstart

Cargo, RustC, GCC, and Makefile **compatibility must be present on the system** if these programs are to be installed.

Every executable in this repository can have the `--help` flag passed to display helpful information.


```
./build.sh -w && ./install.sh -c # This installs the binaries to ~/.local/bin on Unix OS's
```

Or, for Windows

```pwsh
.\build.ps1 -w && .\install.ps1 -c
```

The binaries can be run by calling `basm`, `belle`, or `bdump`.

## Further documentation - [docs](https://github.com/BlueGummi/belle/tree/master/docs)

### ISA (Instruction set) - [docs/isa](https://github.com/BlueGummi/belle/tree/master/docs/isa)

### Assembler - [docs/basm.md](https://github.com/BlueGummi/belle/blob/master/docs/basm.md)

### Emulator - [docs/belle.md](https://github.com/BlueGummi/belle/blob/master/docs/belle.md)

### Disassembler - [docs/bdump.md](https://github.com/BlueGummi/belle/blob/master/docs/bdump.md)

### Utilities - [docs/btils.md](https://github.com/BlueGummi/belle/blob/master/docs/btils.md)

### BELLE and the BELLE utilities in action:
![BELLE Usage GIF](https://github.com/BlueGummi/belle/blob/master/media/belle-usage.gif)


## Naming

**BELLE** is the *emulator*, whilst **BELLE-ISA/ISABELLE** is the *instruction set*.
