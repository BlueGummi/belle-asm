# belle-asm - the assembler for BELLE

## Quickstart

### Prerequisites
`cargo`, `rustc`, and `git` must be installed on the system.



To begin, first clone this repository.

```git clone https://github.com/BlueGummi/belle-asm.git```

Then, `cd` into the directory and build the program from source.

```cd belle-asm && cargo build --release```

To assemble source code, execute this.

```cargo run --release -- -o main main.asm```

Different flags can be passed to make the assembler emit different output, but none will affect how it assembles code.


| Field          | CLI                 | Variable type | Default value | Example    |
| :------------- | :------------------ | :------------ | :-----------: | :--------- |
| Source code    | `file.asm`          | String        | `""`          | `main.asm` |
| Output binary  | `-o` or `--output`  | String        | `"a.out"`     | `-o main`  |
| Verbose output | `-v` or `--verbose` | Boolean       | `false`       | `-v`       |
| Debug output   | `-d` or `--debug`   | Boolean       | `false`       | `-d`       |
| Display tips   | `-t` or `--tips`    | Boolean       | `false`       | `-t`       |
| Display help   | `-h` or `--help`    | Boolean       | `false`       | `-h`       |
