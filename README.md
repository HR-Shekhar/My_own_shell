# Rust Mini Shell

A minimal interactive shell written in **Rust**, built to understand how real shells work at a low level.

This project focuses on correctness, clarity, and learning core shell concepts rather than full feature parity with Bash or Zsh.

---

## Features

### Implemented
- Interactive REPL loop
- Execute external programs via `$PATH`
- Built-in commands:
  - `cd` (supports relative paths and home directory)
  - `pwd`
  - `echo`
  - `type`
  - `exit`
- Shell-style argument parsing
  - Supports single quotes `'...'`
  - Supports double quotes `"..."`
  - Preserves spaces inside quotes
  - Concatenates adjacent quoted and unquoted text
- Proper working directory handling

### Not Implemented
- `ls` (relies on external system `ls` if available)
- Autocompletion
- Command history
- Pipes and redirection
- Environment variable expansion
- Globbing (`*`, `?`)
- Job control

---

## Build & Run

### Requirements
- Rust (stable)
- Cargo

### Run the shell
```bash
cargo run

## Build release binary
cargo build --release

## Example

$ pwd
/home/user

$ echo "hello   world"
hello   world

$ type echo
echo is a shell builtin

$ cd ..
$ pwd
/home