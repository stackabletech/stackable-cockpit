# stackabled

## The stack

Like all our software, the `stackabled` server is written in Rust. The server internally uses the `stackable` library
and exposes functionality via a HTTP REST API. Core community provided dependencies are:

- [`tokio`][tokio-link], the async runtime, which ised used to handle incoming network requests over HTTP/TCP.
- [`axum`][axum-link], a HTTP framework for building REST APIs. It is based on the `tokio` runtime.
- [`utoipa`][utoipa-link], a proc-macro/derive based OpenAPI spec generator which nicely integrates with `axum`.
- [`clap`][clap-link], a powerful command line argument parser which powers the CLI interface.
- [`tracing`][tracing-link] is used for context-rich application logging. Our complete stack emits tracing events which
  can be consumed via multiple different subscribers.
- [`thiserror`][thiserror-link] and [`anyhow`][anyhow-link] for easy custom error handling.

The Stackable UI is bundled into the final binary during the build process. The source code for the frontend is located
in [web](../../web). Detailed information about the frontend stack can be found the the [README](../../web/README.me).

## Development

```shell
cargo run -p stackabled -- <ARGS>
```

```shell
cargo build -p stackabled --release # or --debug
./target/release/stackabled <ARGS>
```

[tokio-link]: https://tokio.rs/
[axum-link]: https://github.com/tokio-rs/axum
[utoipa-link]: https://github.com/juhaku/utoipa
[clap-link]: https://github.com/clap-rs/clap
[tracing-link]: https://github.com/tokio-rs/tracing
[thiserror-link]: https://github.com/Dtolnay/thiserror
[anyhow-link]: https://github.com/dtolnay/anyhow