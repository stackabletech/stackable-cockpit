# stackable

Stackable library powering the `stackablectl` CLI tool and `stackabled` API server. The components are:

- `cluster`: Cluster-related functions and data structures. Currently two kind of cluster orchestration tools are
  supported:
  - kind
  - minikube
- `common`: Some commonly used types, like `List` and `ManifestSpec`
- `platform`: Stackable Data Platform related code:
  - `demo`: Demo related functions and types
  - `operator`: Module containing operator related types and functions to install these components using Helm
  - `release`: Module containing the `ReleaseSpec` and installation methods which install individual operators
  - `stack`: Stacks describe commonly used collections of operators, provides methods to install complete stacks
- `utils`: Various utility functions and helper types like `PathOrUrl` or `read_yaml_data_from_remote`
- `constants`: Constants used across the codebase
- `helm`: A wrapper around the Go Helm library. Provides all Helm-related functions for example for installation of
  charts
- `kube`: Functions which make use of direct Kubernetes API access using the `kube_rs` crate

## Developer setup

### Testing

```shell
cargo test -p stackable
```
