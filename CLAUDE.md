# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

AtCoder Boot Camp for Beginners practice problems ([kenkoooo.com/atcoder#/training](https://kenkoooo.com/atcoder#/training)). Each problem lives under `Easy/NNN/` and may be solved in multiple languages. The `Easy/NNN/README.md` contains a link to the original AtCoder problem.

## Repository Structure

```
Easy/NNN/
  README.md          # link to AtCoder problem
  rust/              # Rust solution (most common)
  C++/               # C++ solution with CMake
  typescript/        # TypeScript solution via Deno
  go/                # Go solution
```

## Language-Specific Commands

### Rust
Uses `proconio` for input parsing. Each problem is a standalone Cargo project.

```sh
cd Easy/NNN/rust
cargo build
cargo run < input.txt
```

### C++
Uses CMake. The build target is always named `MyTarget`.

```sh
cd Easy/NNN/C++
cmake -B build
cmake --build build
echo "input" | ./build/MyTarget
```

### TypeScript (Deno)
```sh
deno run Easy/NNN/typescript/src/main.ts < input.txt
```

### Go
```sh
cd Easy/NNN/go/src
go run main.go < input.txt
```

## Conventions

- Tool versions managed via [asdf](https://asdf-vm.com/) (`.tool-versions`): Go 1.22.12.
- Rust solutions use `proconio::input!` macro for all standard input.
- TypeScript solutions use Deno's `Deno.stdin.readable` async iterator for input.
- C++ `CMakeLists.txt` is templated and handles both macOS (Homebrew) and Linux includes automatically.
- New problems follow the branch naming pattern `easy-NNN-<keyword>`.
