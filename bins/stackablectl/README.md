# stackablectl

The documentation of `stackablectl` can be found in the [documentation of the Stackable Data Platform][ctl-docs].

[ctl-docs]: https://docs.stackable.tech/stackablectl/stable/index.html

## Usage

The `stackablectl` binary provides extensive help and usage information. This help can be displayed for the root as well
as any subcommands using the `--help` flag.

```plain
Usage: stackablectl [OPTIONS] <COMMAND>

Commands:
  operator     Interact with single operator instead of the full platform
  release      Interact with all operators of the platform which are released together
  stack        Interact with stacks, which are ready-to-use product combinations
  stacklets    Interact with deployed stacklets, which are bundles of resources and containers required to run the product
  demo         Interact with demos, which are end-to-end usage demonstrations of the Stackable data platform
  completions  Generate shell completions for this tool
  cache        Interact with locally cached files
  help         Print this message or the help of the given subcommand(s)

Options:
  -l, --log-level <LOG_LEVEL>
          Log level this application uses

      --no-cache
          Do not cache the remote (default) demo, stack and release files

          Cached files are saved at '$XDG_CACHE_HOME/stackablectl', which is usually
          '$HOME/.cache/stackablectl' when not explicitly set.

      --offline
          Do not request any remote files via the network

  -n, --operator-namespace <OPERATOR_NAMESPACE>
          Namespace in the cluster used to deploy the products and operators

          [default: default]

  -d, --demo-file <DEMO_FILES>
          Provide one or more additional (custom) demo file(s)

          Demos are loaded in the following order: Remote (default) demo file, custom
          demo files provided via the 'STACKABLE_DEMO_FILES' environment variable, and
          lastly demo files provided via the '-d/--demo-file' argument(s). If there are
          demos with the same name, the last demo definition will be used.

          Use "stackablectl -d path/to/demos1.yaml -d path/to/demos2.yaml [OPTIONS] <COMMAND>"
          to provide multiple additional demo files.

  -s, --stack-file <STACK_FILES>
          Provide one or more additional (custom) stack file(s)

          Stacks are loaded in the following order: Remote (default) stack file, custom
          stack files provided via the 'STACKABLE_STACK_FILES' environment variable, and
          lastly demo files provided via the '-s/--stack-file' argument(s). If there are
          stacks with the same name, the last stack definition will be used.

          Use "stackablectl -s path/to/stacks1.yaml -s path/to/stacks2.yaml [OPTIONS] <COMMAND>"
          to provide multiple additional stack files.

  -r, --release-file <RELEASE_FILES>
          Provide one or more additional (custom) release file(s)

          Releases are loaded in the following order: Remote (default) release file,
          custom release files provided via the 'STACKABLE_RELEASE_FILES' environment
          variable, and lastly release files provided via the '-r/--release-file'
          argument(s). If there are releases with the same name, the last release
          definition will be used.

          Use "stackablectl -r path/to/releases1.yaml -r path/to/releases2.yaml [OPTIONS] <COMMAND>"
          to provide multiple additional release files.

      --helm-repo-stable <URL>
          Provide a custom Helm stable repository URL

          [default: https://repo.stackable.tech/repository/helm-stable/]

      --helm-repo-test <URL>
          Provide a custom Helm test repository URL

          [default: https://repo.stackable.tech/repository/helm-test/]

      --helm-repo-dev <URL>
          Provide a custom Helm dev repository URL

          [default: https://repo.stackable.tech/repository/helm-dev/]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Dev Setup

### Building

The CLI tool `stackablectl` can be build using Cargo:

```shell
cargo build --release -p stackablectl
```

### Generating man pages and shell completions

The generation of the man pages and the shell completions is part of pre-commit hooks, but can however be run manually
using the following commands:

```shell
cargo xtask gen-comp
cargo xtask gen-man
```
