# If tilt_options.json exists read it and load the default_registry value from it
settings = read_json('tilt_options.json', default={})
registry = settings.get('default_registry', 'oci.stackable.tech/sandbox')

# Configure default registry either read from config file above, or with default value of "oci.stackable.tech/sandbox"
default_registry(registry)

meta = read_json('nix/meta.json')
operator_name = meta['operator']['name']

custom_build(
    registry + '/' + operator_name,
    'make regenerate-nix && nix-build . -A docker --argstr dockerName "${EXPECTED_REGISTRY}/' + operator_name + '" && ./result/load-image | docker load',
    deps=[
      # Rust
      'rust', 'Cargo.toml', 'Cargo.lock', 'vendor',
      # Web UI
      'web', 'yarn.lock',
      # Nix
      'nix', 'default.nix',
    ],
    ignore=['*.~undo-tree~'],
    # ignore=['result*', 'Cargo.nix', 'target', *.yaml],
    outputs_image_ref_to='result/ref',
)

# Load the latest CRDs from Nix
watch_file('result')
# if os.path.exists('result'):
#    k8s_yaml('result/crds.yaml')

# Exclude stale CRDs from Helm chart, and apply the rest
helm_crds, helm_non_crds = filter_yaml(
   helm(
      'deploy/helm/' + operator_name,
      name=operator_name,
      namespace="stackable-operators",
      set=[
         'image.repository=' + registry + '/' + operator_name,
      ],
   ),
   api_version = "^apiextensions\\.k8s\\.io/.*$",
   kind = "^CustomResourceDefinition$",
)
k8s_yaml(helm_non_crds)

k8s_resource(workload='stackable-cockpit-deployment', port_forwards=['8001:8000'])
