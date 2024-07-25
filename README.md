# gen-compile-commands

Utility to generate `compile-commands.json` based on `compile-flags.txt`

```
Usage: gen-compile-commands [OPTIONS] --root <ROOT> --compile-flags <COMPILE_FLAGS> --out <OUT>

Options:
      --root <ROOT>                    Root project directory, all files in `compile_commands.json` will be specified relative to this path
      --compile-flags <COMPILE_FLAGS>  Path to the `compile_flags.txt`. Used for every file in `compile_commands.json`
      --out <OUT>                      Path to the resulting `compile_commands.json`
      --src-dir <SRC_DIR>              Directory to scan for source files
      --ext <EXT>                      Source file extensions to include in `compile_commands.json` [default: c++ cc]
  -h, --help                           Print help
```
