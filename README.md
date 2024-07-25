# gen-compile-commands

Simple utility to generate `compile-commands.json` based on `compile-flags.txt`.

Without `compile-commands.json` clangd can't enumerate project files.
While include/symbol resolution works, find references and other features
that require full project scanning do not work.

`gen-compile-commands` uses single `compile-flags.txt` file as a template
for all source files discovered in `--src-dir`.

## Installation

`cargo install gen-compile-commands`

## Usage

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
