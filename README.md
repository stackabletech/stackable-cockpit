# Stackable Cockpit

⚠️ Notice: This repository and all its components are currently WIP. At this point in time, no stable version is
released. Users of `stackablectl` should continue to use the current latest version. A switch can be made when the time
is appropriate.

This repository contains the Stackable library `stackable`, the Stackable CLI tool `stackablectl`, the Stackable server
`stackabled`, and the web-based admin user interface.

## Components

- [`stackable`][lib-readme]: The underlying library for all actions related to the Stackable Data Platform
- [`stackablectl`][ctl-readme]: CLI tool to interact with local and remote deployments of the data platform
- [`stackabled`][server-readme]: API server used by frontends to interact with the data platform
- [`stackabled-web`][web-readme]: The web-based admin UI powered by SolidJS, TypeScript and Vite

## Developer Setup

### Prerequisites

- A working (and up2date) NodeJS installation, with pnpm as the preferred package manager
- A working (and up2date) Rust installation including rustc, clippy, and cargo
- Optional, but strongly advised: a working [pre-commit][pre-commit] installation

### Getting started

```shell
git clone git@github.com:stackabletech/stackable-cockpit.git
cd stackable-cockpit
```

The admin UI is registered as a crate and is part of the build process, as the HTML/CSS/JS bundle is included in the
final `stackabled` binary. To get the build process running, first execute `pnpm i` to install all required NodeJS
dependencies in the `node_modules` folder.

---

Each component can be build separately like this:

```shell
cargo build --release -p stackablectl             # Builds the stackablectl
cargo build --release -p stackabled               # Builds the Stackable API server
cargo build --release -p stackabled --features ui # Builds the Stackable API server bundled with the admin UI
cd web && pnpm build && cd -                      # Builds the admin UI
```

### Pre-commit hooks and xtasks

This repository uses multiple pre-commit hooks to run checks, formatting and code-generation on different files. The
hooks are:

- [`trailing-whitespace`](https://github.com/pre-commit/pre-commit-hooks#trailing-whitespace): Trims trailing whitespace
  in all files
- [`end-of-file-fixer`](https://github.com/pre-commit/pre-commit-hooks#end-of-file-fixer): Files need to end with
  newlines
- [`detect-aws-credentials`](https://github.com/pre-commit/pre-commit-hooks#detect-aws-credentials): Detect AWS secrets
- [`detect-private-key`](https://github.com/pre-commit/pre-commit-hooks#detect-private-key): Detect private keys
- [`yamllint`](https://github.com/adrienverge/yamllint): Runs linting on all YAML files
- [`markdownlint`](https://github.com/igorshubovych/markdownlint-cli): Runs linting on all Markdown files
- [`prettier`](https://github.com/pre-commit/mirrors-prettier): Runs prettier on files located in `web`
- `cargo clippy -- -D warnings`: Runs Clippy on all files and errors on warnings
- `cargo fmt -- --check`: Checks if Rust code needs formatting
- `cargo xtask gen-comp`: Runs shell completions generation for `stackablectl`
- `cargo xtask gen-mam`: Runs man page generation for `stackablectl`

[server-readme]: ./bins/stackabled/README.md
[ctl-readme]: ./bins/stackablectl/README.md
[pre-commit]: https://pre-commit.com/
[web-readme]: ./web/README.md
[lib-readme]: ./src/README.md
[xtasks]: ./xtask/src/main.rs
