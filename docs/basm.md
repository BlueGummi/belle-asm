# The BELLE assembler - basm

## Quickstart

If the build script **has not been executed yet**, run this.

```
cargo build --release
```

To **assemble source code**, execute this.

```
cargo run --release -- -o output source.asm
```

Or, **if the assembler has been installed**, run 
```
basm -o output source.asm
```
from any directory.

Different flags can be passed to make the assembler **emit different output**, but none will affect how it assembles code.


| Field          | CLI                 | Variable type | Default value | Example    |
| :------------- | :------------------ | :------------ | :-----------: | :--------- |
| Source code    | `file.asm`          | String        | `""`          | `main.asm` |
| Output binary  | `-o` or `--output`  | String        | `"a.out"`     | `-o main`  |
| Verbose output | `-v` or `--verbose` | Boolean       | `false`       | `-v`       |
| Debug output   | `-d` or `--debug`   | Boolean       | `false`       | `-d`       |
| Display tips   | `-t` or `--tips`    | Boolean       | `false`       | `-t`       |
| Display help   | `-h` or `--help`    | Boolean       | `false`       | `-h`       |

# Syntax

## Instruction syntax

The BELLE assembler is **case-agnostic**, as when data is parsed, it either gets **converted to upper or lowercase** for further processing.

All instructions will be formatted **Instruction** destination, **source**.

```asm
    mov %r0, #4 ; this is valid
    mOv %R0, #4 ; this is also valid
    MOV %r0, #4 ; this is valid too
```

Different operands, depending on the type of operand, will have a **different prefixed symbol**.

| Symbol | Meaning | Description | Example |
| :----- | :------ | :------ | :------|
| `#`    | Literal | A literal value to be used as the source for an operation | `#4 ; Literal 4` |
| `%r`   | Register | A register to be used as the source or destination for an operation | `%r3 ; Register 3` | 
| `$`    | Memory address | A memory address to be used as the source or destination for an operation | `$400 ; Memory address 400` |
| `&r`   | Register pointer | A register that contains a memory address that can be accessed by treating the register as a pointer | `&r4 ; Treat the value in register 4 as a memory address and obtain the value at that memory address` | 
| `&$` | Memory address pointer | A memory address whose value is treated as a pointer | `&$10 ; Treat the value in memory address 10 as a pointer and obtain the value at the memory address` | 
| `@`  | Subroutine call | A symbol used to refer to the memory address of a subroutine later in the program | `@foo ; This is replaced with the memory address of the 'foo' subroutine at compile time` | 
| `.` | CPU directive | A one-time directive given to the CPU when the memory is loaded. Expanded upon later | `.ssp $40 ; Set stack pointer to memory address 40` | 


## Assembler directives

The BELLE assembler has a `#include` directive, similar to C/C++, where the user can **specify a file to include to the top** of the file, allowing for projects to be split across multiple directories and many files.



The assembler will **only read the include directive from the main file that is to be assembled**, and once a line after the first line is no longer an `#include`, the assembler will stop trying to look for the directive, and simply throw an error if an `#include` is found.


## CPU directives


The BELLE-ISA allows for **parts of the CPU to be adjusted based on certain directives that it receives**. The parts that it changes are only changed when the program is loaded into memory, and at **runtime the changes will not be made**.

| Directive | Property changed | Description | Example |
| :----     | :----            | :----:      | :-----  |
| `.ssp`    | Stack pointer    | `.ssp` (Set Stack Pointer) changes the stack pointer's initial value | `.ssp $100` |
| `.sbp`    | Base pointer    | `.sbp` (Set Base Pointer) changes the base pointer's initial value | `.sbp $100` |


# Errors and debugging

## Error emission reasons


The assembler is *very* lenient with arguments passed to each operation (ADD can take subroutines as arguments, JZ can take register values, etc.), however, it can **still emit an error**.


If the code passed to the assembler **contains an error, it will stop assembling**, emit the error, and exit.


The following is a list of possible reasons for the assembler to emit an error.
 - A register value is **too big**
 - A **non-valid syntactical** token is found
 - A subroutine that is being called is **not present in the code**
 - An invalid instruction is **found in the code**
 - A memory address is **too large** (physically cannot be encoded into 16-bit instructions)
 - A literal value is **too large**
 - An instruction that doesn't have the correct amount of arguments

## Debugging source code


The assembler may emit an error **depending on whether or not the code's syntax is valid**. Refer to [docs/isa](https://github.com/BlueGummi/belle/tree/master/docs/isa) to view the **ISA and syntax** for the assembly code.


If the error happened at the syntax symbol and **token validation stage** (the lexer), the assembler will also print a red carrot (^) pointing to the **location of the error** in the line that contained an error.


The assembler will only emit **one error** (for brevity's sake) and will emit the first one it sees.


Passing **certain flags to the assembler**, such as `-d` or `-v` will emit different output.


The `-d` flag will display the entire process of **assembling source code**, and will show every token that the **assembler lexes from the input file**. The `-v` flag will create verbose output, **allowing examination of the binary output** for every line, if interested.


The assembler can also emit tips for any **instance of invalid syntax**, and a bug report/issue/PR can be opened if an idea for better tip messages comes to mind for certain errors.


# Other

## Inspecting output


