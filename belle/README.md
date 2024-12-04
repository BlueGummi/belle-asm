# belle - The Big Endian, Low Level Emulator


### This document is a short overview of the emulator. For further documentation, visit [docs/belle.md](https://github.com/BlueGummi/belle/blob/master/docs/belle.md)

## Quickstart


```cargo build --release```

To run source code, execute this

```cargo run --release -- input```

Different flags can be passed to make the emulator emit different output or function differently.


| Field          | CLI                 | Variable type | Default value | Example    |
| :------------- | :------------------ | :------------ | :-----------: | :--------- |
| Binary         | `file`              | String        | `""`          | `main`     |
| Verbose output | `-v` or `--verbose` | Boolean       | `false`       | `-v`       |
| Enter debugger | `-d` or `--debug`   | Boolean       | `false`       | `-d`       |
| Quiet mode     | `-q` or `--quiet`   | Boolean       | `false`       | `-q`       |
| Time delay (ms) | `-t` or `--time-delay`    | Integer       | `0`       | `-t 50`       |
| Don't crash    | `-c` or `--dont-crash` | Boolean | `false` | `-c` |

