module completions {

  # Command line tool to interact with the Stackable Data Platform
  export extern stackablectl [
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Interact with single operator instead of the full platform
  export extern "stackablectl operator" [
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  def "nu-complete stackablectl operator list output_type" [] {
    [ "plain" "table" "json" "yaml" ]
  }

  def "nu-complete stackablectl operator list chart_source" [] {
    [ "oci" "repo" ]
  }

  # List available operators
  export extern "stackablectl operator list" [
    --output(-o): string@"nu-complete stackablectl operator list output_type"
    --chart-source: string@"nu-complete stackablectl operator list chart_source"
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  def "nu-complete stackablectl operator describe output_type" [] {
    [ "plain" "table" "json" "yaml" ]
  }

  def "nu-complete stackablectl operator describe chart_source" [] {
    [ "oci" "repo" ]
  }

  # Print out detailed operator information
  export extern "stackablectl operator describe" [
    OPERATOR: string          # Operator to describe
    --output(-o): string@"nu-complete stackablectl operator describe output_type"
    --chart-source: string@"nu-complete stackablectl operator describe chart_source"
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  def "nu-complete stackablectl operator install cluster_type" [] {
    [ "kind" "minikube" ]
  }

  def "nu-complete stackablectl operator install chart_source" [] {
    [ "oci" "repo" ]
  }

  # Install one or more operators
  export extern "stackablectl operator install" [
    ...OPERATORS: string      # Operator(s) to install
    --operator-namespace: string # Namespace in the cluster used to deploy the operators
    --operator-ns: string     # Namespace in the cluster used to deploy the operators
    --cluster(-c): string@"nu-complete stackablectl operator install cluster_type" # Type of local cluster to use for testing
    --cluster-name: string    # Name of the local cluster
    --cluster-nodes: string   # Number of total nodes in the local cluster
    --cluster-cp-nodes: string # Number of control plane nodes in the local cluster
    --chart-source: string@"nu-complete stackablectl operator install chart_source"
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Uninstall one or more operators
  export extern "stackablectl operator uninstall" [
    ...operators: string      # One or more operators to uninstall
    --operator-namespace: string # Namespace in the cluster used to deploy the operators
    --operator-ns: string     # Namespace in the cluster used to deploy the operators
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  def "nu-complete stackablectl operator installed output_type" [] {
    [ "plain" "table" "json" "yaml" ]
  }

  # List installed operators
  export extern "stackablectl operator installed" [
    --output(-o): string@"nu-complete stackablectl operator installed output_type"
    --operator-namespace: string # Namespace in the cluster used to deploy the operators
    --operator-ns: string     # Namespace in the cluster used to deploy the operators
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "stackablectl operator help" [
  ]

  # List available operators
  export extern "stackablectl operator help list" [
  ]

  # Print out detailed operator information
  export extern "stackablectl operator help describe" [
  ]

  # Install one or more operators
  export extern "stackablectl operator help install" [
  ]

  # Uninstall one or more operators
  export extern "stackablectl operator help uninstall" [
  ]

  # List installed operators
  export extern "stackablectl operator help installed" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "stackablectl operator help help" [
  ]

  # Interact with all operators of the platform which are released together
  export extern "stackablectl release" [
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  def "nu-complete stackablectl release list output_type" [] {
    [ "plain" "table" "json" "yaml" ]
  }

  # List available releases
  export extern "stackablectl release list" [
    --output(-o): string@"nu-complete stackablectl release list output_type"
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  def "nu-complete stackablectl release describe output_type" [] {
    [ "plain" "table" "json" "yaml" ]
  }

  # Print out detailed release information
  export extern "stackablectl release describe" [
    RELEASE: string
    --output(-o): string@"nu-complete stackablectl release describe output_type"
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  def "nu-complete stackablectl release install cluster_type" [] {
    [ "kind" "minikube" ]
  }

  def "nu-complete stackablectl release install chart_source" [] {
    [ "oci" "repo" ]
  }

  # Install a specific release
  export extern "stackablectl release install" [
    RELEASE: string           # Release to install
    --include(-i): string     # Whitelist of product operators to install
    --exclude(-e): string     # Blacklist of product operators to install
    --operator-namespace: string # Namespace in the cluster used to deploy the operators
    --operator-ns: string     # Namespace in the cluster used to deploy the operators
    --cluster(-c): string@"nu-complete stackablectl release install cluster_type" # Type of local cluster to use for testing
    --cluster-name: string    # Name of the local cluster
    --cluster-nodes: string   # Number of total nodes in the local cluster
    --cluster-cp-nodes: string # Number of control plane nodes in the local cluster
    --chart-source: string@"nu-complete stackablectl release install chart_source"
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Uninstall a release
  export extern "stackablectl release uninstall" [
    RELEASE: string           # Name of the release to uninstall
    --operator-namespace: string # Namespace in the cluster used to deploy the operators
    --operator-ns: string     # Namespace in the cluster used to deploy the operators
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "stackablectl release help" [
  ]

  # List available releases
  export extern "stackablectl release help list" [
  ]

  # Print out detailed release information
  export extern "stackablectl release help describe" [
  ]

  # Install a specific release
  export extern "stackablectl release help install" [
  ]

  # Uninstall a release
  export extern "stackablectl release help uninstall" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "stackablectl release help help" [
  ]

  # Interact with stacks, which are ready-to-use product combinations
  export extern "stackablectl stack" [
    --release: string         # Target a specific Stackable release
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  def "nu-complete stackablectl stack list output_type" [] {
    [ "plain" "table" "json" "yaml" ]
  }

  # List available stacks
  export extern "stackablectl stack list" [
    --output(-o): string@"nu-complete stackablectl stack list output_type"
    --release: string         # Target a specific Stackable release
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  def "nu-complete stackablectl stack describe output_type" [] {
    [ "plain" "table" "json" "yaml" ]
  }

  # Describe a specific stack
  export extern "stackablectl stack describe" [
    stack_name: string        # Name of the stack to describe
    --output(-o): string@"nu-complete stackablectl stack describe output_type"
    --release: string         # Target a specific Stackable release
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  def "nu-complete stackablectl stack install cluster_type" [] {
    [ "kind" "minikube" ]
  }

  def "nu-complete stackablectl stack install chart_source" [] {
    [ "oci" "repo" ]
  }

  # Install a specific stack
  export extern "stackablectl stack install" [
    stack_name: string        # Name of the stack to describe
    --skip-release            # Skip the installation of the release during the stack install process
    --stack-parameters: string # List of parameters to use when installing the stack
    --parameters: string      # List of parameters to use when installing the stack
    --cluster(-c): string@"nu-complete stackablectl stack install cluster_type" # Type of local cluster to use for testing
    --cluster-name: string    # Name of the local cluster
    --cluster-nodes: string   # Number of total nodes in the local cluster
    --cluster-cp-nodes: string # Number of control plane nodes in the local cluster
    --operator-namespace: string # Namespace where the operators are deployed
    --operator-ns: string     # Namespace where the operators are deployed
    --product-namespace(-n): string # Namespace where the products (e.g. stacks or demos) are deployed
    --product-ns: string      # Namespace where the products (e.g. stacks or demos) are deployed
    --chart-source: string@"nu-complete stackablectl stack install chart_source"
    --release: string         # Target a specific Stackable release
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "stackablectl stack help" [
  ]

  # List available stacks
  export extern "stackablectl stack help list" [
  ]

  # Describe a specific stack
  export extern "stackablectl stack help describe" [
  ]

  # Install a specific stack
  export extern "stackablectl stack help install" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "stackablectl stack help help" [
  ]

  # Interact with deployed stacklets, which are bundles of resources and containers required to run the product
  export extern "stackablectl stacklet" [
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Display credentials for a stacklet
  export extern "stackablectl stacklet credentials" [
    product_name: string      # The name of the product, for example 'superset'
    stacklet_name: string     # The name of the stacklet, for example 'superset'
    --product-namespace(-n): string # Namespace in the cluster used to deploy the products
    --product-ns: string      # Namespace in the cluster used to deploy the products
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  def "nu-complete stackablectl stacklet list output_type" [] {
    [ "plain" "table" "json" "yaml" ]
  }

  # List deployed stacklets
  export extern "stackablectl stacklet list" [
    --output(-o): string@"nu-complete stackablectl stacklet list output_type"
    --operator-namespace: string # Namespace where the operators are deployed
    --operator-ns: string     # Namespace where the operators are deployed
    --product-namespace(-n): string # Namespace where the products (e.g. stacks or demos) are deployed
    --product-ns: string      # Namespace where the products (e.g. stacks or demos) are deployed
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "stackablectl stacklet help" [
  ]

  # Display credentials for a stacklet
  export extern "stackablectl stacklet help credentials" [
  ]

  # List deployed stacklets
  export extern "stackablectl stacklet help list" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "stackablectl stacklet help help" [
  ]

  # Interact with demos, which are end-to-end usage demonstrations of the Stackable data platform
  export extern "stackablectl demo" [
    --release: string         # Target a specific Stackable release
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  def "nu-complete stackablectl demo list output_type" [] {
    [ "plain" "table" "json" "yaml" ]
  }

  # List available demos
  export extern "stackablectl demo list" [
    --output(-o): string@"nu-complete stackablectl demo list output_type"
    --release: string         # Target a specific Stackable release
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  def "nu-complete stackablectl demo describe output_type" [] {
    [ "plain" "table" "json" "yaml" ]
  }

  # Print out detailed demo information
  export extern "stackablectl demo describe" [
    DEMO: string              # Demo to describe
    --output(-o): string@"nu-complete stackablectl demo describe output_type"
    --release: string         # Target a specific Stackable release
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  def "nu-complete stackablectl demo install cluster_type" [] {
    [ "kind" "minikube" ]
  }

  def "nu-complete stackablectl demo install chart_source" [] {
    [ "oci" "repo" ]
  }

  # Install a specific demo
  export extern "stackablectl demo install" [
    DEMO: string              # Demo to install
    --skip-release            # Skip the installation of the release during the stack install process
    --stack-parameters: string # List of parameters to use when installing the stack
    --parameters: string      # List of parameters to use when installing the demo
    --cluster(-c): string@"nu-complete stackablectl demo install cluster_type" # Type of local cluster to use for testing
    --cluster-name: string    # Name of the local cluster
    --cluster-nodes: string   # Number of total nodes in the local cluster
    --cluster-cp-nodes: string # Number of control plane nodes in the local cluster
    --operator-namespace: string # Namespace where the operators are deployed
    --operator-ns: string     # Namespace where the operators are deployed
    --product-namespace(-n): string # Namespace where the products (e.g. stacks or demos) are deployed
    --product-ns: string      # Namespace where the products (e.g. stacks or demos) are deployed
    --chart-source: string@"nu-complete stackablectl demo install chart_source" # Source the charts from either a OCI registry or from index.yaml-based repositories
    --release: string         # Target a specific Stackable release
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "stackablectl demo help" [
  ]

  # List available demos
  export extern "stackablectl demo help list" [
  ]

  # Print out detailed demo information
  export extern "stackablectl demo help describe" [
  ]

  # Install a specific demo
  export extern "stackablectl demo help install" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "stackablectl demo help help" [
  ]

  # Generate shell completions for this tool
  export extern "stackablectl completions" [
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Generate shell completions for Bash
  export extern "stackablectl completions bash" [
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Generate shell completions for Elvish
  export extern "stackablectl completions elvish" [
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Generate shell completions for Fish
  export extern "stackablectl completions fish" [
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Generate shell completions for Nushell
  export extern "stackablectl completions nushell" [
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Generate shell completions for ZSH
  export extern "stackablectl completions zsh" [
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "stackablectl completions help" [
  ]

  # Generate shell completions for Bash
  export extern "stackablectl completions help bash" [
  ]

  # Generate shell completions for Elvish
  export extern "stackablectl completions help elvish" [
  ]

  # Generate shell completions for Fish
  export extern "stackablectl completions help fish" [
  ]

  # Generate shell completions for Nushell
  export extern "stackablectl completions help nushell" [
  ]

  # Generate shell completions for ZSH
  export extern "stackablectl completions help zsh" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "stackablectl completions help help" [
  ]

  # Interact with locally cached files
  export extern "stackablectl cache" [
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # List cached files
  export extern "stackablectl cache list" [
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Clean cached files
  export extern "stackablectl cache clean" [
    --old                     # Only remove outdated files in the cache
    --outdated                # Only remove outdated files in the cache
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "stackablectl cache help" [
  ]

  # List cached files
  export extern "stackablectl cache help list" [
  ]

  # Clean cached files
  export extern "stackablectl cache help clean" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "stackablectl cache help help" [
  ]

  # EXPERIMENTAL: Launch a debug container for a Pod
  export extern "stackablectl experimental-debug" [
    --namespace(-n): string   # The namespace of the Pod being debugged
    pod: string               # The Pod to debug
    --container(-c): string   # The target container to debug
    --image: string           # The debug container image
    ...cmd: string            # The command to run in the debug container
    --log-level(-l): string   # Log level this application uses
    --no-cache                # Do not cache the remote (default) demo, stack and release files
    --demo-file(-d): string   # Provide one or more additional (custom) demo file(s)
    --stack-file(-s): string  # Provide one or more additional (custom) stack file(s)
    --release-file(-r): string # Provide one or more additional (custom) release file(s)
    --helm-repo-stable: string # Provide a custom Helm stable repository URL
    --helm-repo-test: string  # Provide a custom Helm test repository URL
    --helm-repo-dev: string   # Provide a custom Helm dev repository URL
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "stackablectl help" [
  ]

  # Interact with single operator instead of the full platform
  export extern "stackablectl help operator" [
  ]

  # List available operators
  export extern "stackablectl help operator list" [
  ]

  # Print out detailed operator information
  export extern "stackablectl help operator describe" [
  ]

  # Install one or more operators
  export extern "stackablectl help operator install" [
  ]

  # Uninstall one or more operators
  export extern "stackablectl help operator uninstall" [
  ]

  # List installed operators
  export extern "stackablectl help operator installed" [
  ]

  # Interact with all operators of the platform which are released together
  export extern "stackablectl help release" [
  ]

  # List available releases
  export extern "stackablectl help release list" [
  ]

  # Print out detailed release information
  export extern "stackablectl help release describe" [
  ]

  # Install a specific release
  export extern "stackablectl help release install" [
  ]

  # Uninstall a release
  export extern "stackablectl help release uninstall" [
  ]

  # Interact with stacks, which are ready-to-use product combinations
  export extern "stackablectl help stack" [
  ]

  # List available stacks
  export extern "stackablectl help stack list" [
  ]

  # Describe a specific stack
  export extern "stackablectl help stack describe" [
  ]

  # Install a specific stack
  export extern "stackablectl help stack install" [
  ]

  # Interact with deployed stacklets, which are bundles of resources and containers required to run the product
  export extern "stackablectl help stacklet" [
  ]

  # Display credentials for a stacklet
  export extern "stackablectl help stacklet credentials" [
  ]

  # List deployed stacklets
  export extern "stackablectl help stacklet list" [
  ]

  # Interact with demos, which are end-to-end usage demonstrations of the Stackable data platform
  export extern "stackablectl help demo" [
  ]

  # List available demos
  export extern "stackablectl help demo list" [
  ]

  # Print out detailed demo information
  export extern "stackablectl help demo describe" [
  ]

  # Install a specific demo
  export extern "stackablectl help demo install" [
  ]

  # Generate shell completions for this tool
  export extern "stackablectl help completions" [
  ]

  # Generate shell completions for Bash
  export extern "stackablectl help completions bash" [
  ]

  # Generate shell completions for Elvish
  export extern "stackablectl help completions elvish" [
  ]

  # Generate shell completions for Fish
  export extern "stackablectl help completions fish" [
  ]

  # Generate shell completions for Nushell
  export extern "stackablectl help completions nushell" [
  ]

  # Generate shell completions for ZSH
  export extern "stackablectl help completions zsh" [
  ]

  # Interact with locally cached files
  export extern "stackablectl help cache" [
  ]

  # List cached files
  export extern "stackablectl help cache list" [
  ]

  # Clean cached files
  export extern "stackablectl help cache clean" [
  ]

  # EXPERIMENTAL: Launch a debug container for a Pod
  export extern "stackablectl help experimental-debug" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "stackablectl help help" [
  ]

}

export use completions *
