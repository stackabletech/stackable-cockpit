# stackablectl

The documentation of `stackablectl` can be found in the [documentation of the Stackable Data Platform][ctl-docs].

[ctl-docs]: https://docs.stackable.tech/stackablectl/stable/index.html

## Usage

TODO

## Building

The CLI tool `stackablectl` can be build using Cargo:

```shell
cargo build --release -p stackablectl
```

### Man Pages

This generates man pages in `extra/man`:

```shell
cargo xtask gen-man
```

### Shell Completions

This generates shell completions in `extra/completions`. Currently, we generate completions for Bash, Fish and ZSH.

```shell
cargo xtask gen-comp
```
