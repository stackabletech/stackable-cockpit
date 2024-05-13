# helm-sys

This crate provides bindings to a Helm client library written in Go, which can
be used from Rust via the FFI/cgo interface. This way, Rust can call Go code to
handle various Helm related tasks.

## Troubleshooting

If during build, an error message like `fatal error: 'stddef.h' file not found`
is encountered, this is usually an ididcator that `clang` is not installed or
configured properly. In case of not being installed, just install the package
with your package manager of choice, for example:

```shell
apt install clang
```
