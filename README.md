# BELLE - The complete program utility set for the Big Endian, Low Level Emulator

## Quickstart

Cargo, RustC, GCC, and Makefile compatibility must be present on the system if these programs are to be installed.

Every executable in this repository can have the --help flag passed to display helpful information.

Two shell scripts come included within this repository. To build and install the BELLE-assembler and BELLE-disassembler, run

```./build.sh -w && ./install.sh -c # This installs the binaries to ~/.local/bin```

The binaries can be run by calling `basm` or `bdump`.
