# Warp Directory

<p align="center">
<img src="https://img.shields.io/crates/l/warp-dir.svg" alt="license">
<a href="https://crates.io/crates/warp-dir">
<img src="https://img.shields.io/crates/v/warp-dir.svg?colorB=319e8c" alt="Version info">
</a>
<a href="https://github.com/DanCardin/warp-dir/actions?query=workflow%3ATest">
<img src="https://github.com/DanCardin/warp-dir/workflows/Test/badge.svg" alt="Build Status">
</a> <a href="https://codecov.io/gh/DanCardin/warp-dir">
<img src="https://codecov.io/gh/DanCardin/warp-dir/branch/main/graph/badge.svg?token=U7NQIWXWKW"/>
</a><br>
</p>

A [warp-dir](https://github.com/kigster/warp-dir)-like tool, written in Rust.

> Think of this as a folder-navigation super-charge tool that you’d use on a
> most frequently-used set of folders. This becomes really useful if you are
> often finding yourself going into a small number of deeply nested folders with
> a long path prefix.

## Usage

```bash
/home/me/a/b/c/d/> wd add foo
/home/me> cd
/home/me> z foo
/home/me/a/b/c/d/> # Huzzah!
/home/me/a/b/c/d/> wd help

Usage: wd [OPTIONS] [target] [PATH] [COMMAND]

Commands:
  add     Adds the current working directory to your warp points
  remove  Removes the given warp point
  list    Print all stored warp points
  show    Show the path to given warp point (pwd)
  clean   Remove points warping to nonexistent directories
  shell   Subcommands for hooking into the shell
  help    Print this message or the help of the given subcommand(s)

Arguments:
  [target]
          Warps to the directory specified by the warp point

          Possible values:
          - foo:       /home/me/a/b/c/d
```

## Why?

Why rewrite it in Rust?

- for fun!
- I encountered some issues, semiregularly, where it would fail to load/execute
  correctly until opening a new shell (presumably due to it being written in
  bash).
- Additional shell support

### Differences

Essentially, this tool should act like a drop-in replacement for
[warp-dir](https://github.com/kigster/warp-dir) in most cases.

At time of writing, most commands are implemented, although the CLI options will
not work identically.

Notably, the default config lookup location is different! By default it will
look at `$XDG_DATA_HOME/wd/config`. This can be configured with the `$WD_CONFIG`
environment variable, which is also respected by the original warp-dir project.

## Shell Support

Currently explicitly supported shells include: `zsh`, `bash`, and `fish`. The
scaffolding exists to support other shells, which should make supporting other
common shells that might require `"$SHELL"` specific behavior.

Changing the shell's directory requires a minimal amount of shell code to be
executed, so after installing the binary (suggestion below), you will need to
add add a hook to your bashrc/zshrc/config.fish, etc.

- bash `eval "$(wd --shell bash shell init)"`
- zsh `eval "$(wd --shell zsh shell init)"`
- fish `wd --shell fish shell init | source`

Depending on the level of similarity to the above shells, you may be able to get
away with using one of the above `shell init` hooks until explicit support is
added

### Completions

Additionally, you can generate CLI completions with `wd shell completion`, and
write them to the appropriate location for your shell.

This should enable autocompletion of the warp directory target names!

## Installation

### With Cargo

```bash
cargo install warp-dir
```

### Download Release

- Download a pre-built binary from
  [Releases](https://github.com/DanCardin/warp-dir/releases)
