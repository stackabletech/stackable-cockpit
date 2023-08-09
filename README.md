# Stackable Cockpit

⚠️ Notice: This repository and all its components are currently WIP. At this point in time, no stable version is
released. Users of `stackablectl` should continue to use the current latest version. A switch can be made when the time
is appropriate.

This repository contains the Stackable library `stackable-cockpit`, the Stackable CLI tool
`stackablectl`, the Stackable server `stackable-cockpitd`, and the web-based admin user
interface.

## Components

- [`stackable-cockpit`][lib-readme]: The underlying library for all actions related to the Stackable Data Platform
- [`stackablectl`][ctl-readme]: CLI tool to interact with local and remote deployments of the data platform
- [`stackable-cockpitd`][server-readme]: API server used by frontends to interact with the data platform
- [`stackable-cockpit-web`][web-readme]: The web-based admin UI powered by SolidJS, TypeScript and Vite

## Developer Setup

### Prerequisites

- A working (and up2date) NodeJS installation (`v18, lts/hydrogen`), with Yarn as the preferred package manager
- A working (and up2date) Rust installation including rustc, clippy, and cargo
- Optional, but strongly advised: a working [pre-commit][pre-commit] installation. You can find instructions in the pre-commit section on how to get it working.

Don't forget to install global node dependencies:

```
$ npm install -g yarn typescript
```

### Getting started

```shell
git clone git@github.com:stackabletech/stackable-cockpit.git
cd stackable-cockpit
```

Before continuing with the build process, first execute `yarn install` to install all required NodeJS
dependencies in the `node_modules` folder.

The admin UI is registered as a crate and is part of the build process, as the HTML/CSS/JS bundle is included in the final `stackable-cockpitd` binary.

---

Each component can be build separately like this:

```shell
cargo build --release -p stackablectl                     # Builds stackablectl
cargo build --release -p stackable-cockpitd               # Builds the Stackable Cockpit API server
cargo build --release -p stackable-cockpitd --features ui # Builds the Stackable Cockpit API server bundled with the admin UI
cd web && yarn run build && cd -                          # Builds the admin UI
```

### Pre-commit hooks and xtasks

This repository uses multiple pre-commit hooks to run checks, formatting and code-generation on different files.

For pre-commit, please install the tool on your machine. You can install the hooks by running
`pre-commit install` in the project directory. You can try executing all
hooks with `pre-commit run`.

The hooks are:

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
- `cargo xtask gen-openapi`: Runs OpenAPI spec generation to connect backend with frontend
- `cargo xtask gen-ctl-readme`: Generates and inserts `stackablectl` help text into README

[server-readme]: ./rust/stackable-cockpitd/README.md
[ctl-readme]: ./rust/stackablectl/README.md
[pre-commit]: https://pre-commit.com/
[web-readme]: ./web/README.md
[lib-readme]: ./rust/stackable-cockpit/README.md
[xtasks]: ./xtask/src/main.rs
