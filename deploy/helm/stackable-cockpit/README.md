# Helm Chart for Stackable Operator for Stackable Cockpit

This Helm Chart can be used to install Custom Resource Definitions and the Operator for Stackable Cockpit provided by Stackable.

## Requirements

- Create a [Kubernetes Cluster](../Readme.md)
- Install [Helm](https://helm.sh/docs/intro/install/)

## Install the Stackable Operator for Stackable Cockpit

```bash
# From the root of the operator repository
make compile-chart

helm install stackable-cockpit deploy/helm/stackable-cockpit
```

## Usage of the CRDs

The usage of this operator and its CRDs is described in the [documentation](https://docs.stackable.tech/stackable-cockpit/index.html)

The operator has example requests included in the [`/examples`](https://github.com/stackabletech/stackable-cockpit/tree/main/examples) directory.

## Links

[Source](https://github.com/stackabletech/stackable-cockpit)
