# bdump - The disassembler for BELLE

### This document is a short overview of the disassembler. For further documentation, visit [docs/bdump.md](https://github.com/BlueGummi/belle/blob/master/docs/bdump.md)

## Quickstart


```make```

To disassemble source code, execute this.

```./bdump main.asm```

Different flags can be passed to make the disassembler emit different output.


| Field          | CLI                 | Variable type | Default value | Example    |
| :------------- | :------------------ | :------------ | :-----------: | :--------- |
| Source code    | `file.asm`          | String        | `""`          | `main.asm` |
| Verbose output | `-v` or `--verbose` | Boolean       | `false`       | `-v`       |
| Debug output   | `-d` or `--debug`   | Boolean       | `false`       | `-d`       |
| Display binary   | `-b` or `--binary`    | Boolean       | `false`       | `-b`       |
| Display line numbers | `-l` or `--line-num` | Boolean | false | `-l` |
| Display colors | `-c` or `--colors` | Boolean | false | `-c` |
