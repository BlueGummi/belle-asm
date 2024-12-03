# basm - The assembler for BELLE

## Quickstart


```cargo build --release```

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
