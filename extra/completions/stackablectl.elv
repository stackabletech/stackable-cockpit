
use builtin;
use str;

set edit:completion:arg-completer[stackablectl] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'stackablectl'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'stackablectl'= {
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
            cand operator 'Interact with single operator instead of the full platform'
            cand release 'Interact with all operators of the platform which are released together'
            cand stack 'Interact with stacks, which are ready-to-use product combinations'
            cand stacklet 'Interact with deployed stacklets, which are bundles of resources and containers required to run the product'
            cand demo 'Interact with demos, which are end-to-end usage demonstrations of the Stackable data platform'
            cand completions 'Generate shell completions for this tool'
            cand cache 'Interact with locally cached files'
            cand experimental-debug 'EXPERIMENTAL: Launch a debug container for a Pod'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'stackablectl;operator'= {
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
            cand list 'List available operators'
            cand describe 'Print out detailed operator information'
            cand install 'Install one or more operators'
            cand uninstall 'Uninstall one or more operators'
            cand installed 'List installed operators'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'stackablectl;operator;list'= {
            cand -o 'o'
            cand --output 'output'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;operator;describe'= {
            cand -o 'o'
            cand --output 'output'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;operator;install'= {
            cand --operator-namespace 'Namespace in the cluster used to deploy the operators'
            cand --operator-ns 'Namespace in the cluster used to deploy the operators'
            cand -c 'Type of local cluster to use for testing'
            cand --cluster 'Type of local cluster to use for testing'
            cand --cluster-name 'Name of the local cluster'
            cand --cluster-nodes 'Number of total nodes in the local cluster'
            cand --cluster-cp-nodes 'Number of control plane nodes in the local cluster'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;operator;uninstall'= {
            cand --operator-namespace 'Namespace in the cluster used to deploy the operators'
            cand --operator-ns 'Namespace in the cluster used to deploy the operators'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;operator;installed'= {
            cand -o 'o'
            cand --output 'output'
            cand --operator-namespace 'Namespace in the cluster used to deploy the operators'
            cand --operator-ns 'Namespace in the cluster used to deploy the operators'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;operator;help'= {
            cand list 'List available operators'
            cand describe 'Print out detailed operator information'
            cand install 'Install one or more operators'
            cand uninstall 'Uninstall one or more operators'
            cand installed 'List installed operators'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'stackablectl;operator;help;list'= {
        }
        &'stackablectl;operator;help;describe'= {
        }
        &'stackablectl;operator;help;install'= {
        }
        &'stackablectl;operator;help;uninstall'= {
        }
        &'stackablectl;operator;help;installed'= {
        }
        &'stackablectl;operator;help;help'= {
        }
        &'stackablectl;release'= {
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
            cand list 'List available releases'
            cand describe 'Print out detailed release information'
            cand install 'Install a specific release'
            cand uninstall 'Uninstall a release'
            cand upgrade 'Upgrade a release'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'stackablectl;release;list'= {
            cand -o 'o'
            cand --output 'output'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;release;describe'= {
            cand -o 'o'
            cand --output 'output'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;release;install'= {
            cand -i 'Whitelist of product operators to install'
            cand --include 'Whitelist of product operators to install'
            cand -e 'Blacklist of product operators to install'
            cand --exclude 'Blacklist of product operators to install'
            cand --operator-namespace 'Namespace in the cluster used to deploy the operators'
            cand --operator-ns 'Namespace in the cluster used to deploy the operators'
            cand -c 'Type of local cluster to use for testing'
            cand --cluster 'Type of local cluster to use for testing'
            cand --cluster-name 'Name of the local cluster'
            cand --cluster-nodes 'Number of total nodes in the local cluster'
            cand --cluster-cp-nodes 'Number of control plane nodes in the local cluster'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;release;uninstall'= {
            cand --operator-namespace 'Namespace in the cluster used to deploy the operators'
            cand --operator-ns 'Namespace in the cluster used to deploy the operators'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;release;upgrade'= {
            cand -i 'List of product operators to upgrade'
            cand --include 'List of product operators to upgrade'
            cand -e 'Blacklist of product operators to install'
            cand --exclude 'Blacklist of product operators to install'
            cand --operator-namespace 'Namespace in the cluster used to deploy the operators'
            cand --operator-ns 'Namespace in the cluster used to deploy the operators'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;release;help'= {
            cand list 'List available releases'
            cand describe 'Print out detailed release information'
            cand install 'Install a specific release'
            cand uninstall 'Uninstall a release'
            cand upgrade 'Upgrade a release'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'stackablectl;release;help;list'= {
        }
        &'stackablectl;release;help;describe'= {
        }
        &'stackablectl;release;help;install'= {
        }
        &'stackablectl;release;help;uninstall'= {
        }
        &'stackablectl;release;help;upgrade'= {
        }
        &'stackablectl;release;help;help'= {
        }
        &'stackablectl;stack'= {
            cand --release 'Target a specific Stackable release'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
            cand list 'List available stacks'
            cand describe 'Describe a specific stack'
            cand install 'Install a specific stack'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'stackablectl;stack;list'= {
            cand -o 'o'
            cand --output 'output'
            cand --release 'Target a specific Stackable release'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;stack;describe'= {
            cand -o 'o'
            cand --output 'output'
            cand --release 'Target a specific Stackable release'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;stack;install'= {
            cand --stack-parameters 'List of parameters to use when installing the stack'
            cand --parameters 'List of parameters to use when installing the stack'
            cand -c 'Type of local cluster to use for testing'
            cand --cluster 'Type of local cluster to use for testing'
            cand --cluster-name 'Name of the local cluster'
            cand --cluster-nodes 'Number of total nodes in the local cluster'
            cand --cluster-cp-nodes 'Number of control plane nodes in the local cluster'
            cand --operator-namespace 'Namespace where the operators are deployed'
            cand --operator-ns 'Namespace where the operators are deployed'
            cand -n 'Namespace where the stacks or demos are deployed'
            cand --namespace 'Namespace where the stacks or demos are deployed'
            cand --product-ns 'Namespace where the stacks or demos are deployed'
            cand --release 'Target a specific Stackable release'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --skip-release 'Skip the installation of the release during the stack install process'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;stack;help'= {
            cand list 'List available stacks'
            cand describe 'Describe a specific stack'
            cand install 'Install a specific stack'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'stackablectl;stack;help;list'= {
        }
        &'stackablectl;stack;help;describe'= {
        }
        &'stackablectl;stack;help;install'= {
        }
        &'stackablectl;stack;help;help'= {
        }
        &'stackablectl;stacklet'= {
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
            cand credentials 'Display credentials for a stacklet'
            cand list 'List deployed stacklets'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'stackablectl;stacklet;credentials'= {
            cand -n 'Namespace in the cluster used to deploy the products'
            cand --namespace 'Namespace in the cluster used to deploy the products'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;stacklet;list'= {
            cand -o 'o'
            cand --output 'output'
            cand --operator-namespace 'Namespace where the operators are deployed'
            cand --operator-ns 'Namespace where the operators are deployed'
            cand -n 'Namespace where the stacks or demos are deployed'
            cand --namespace 'Namespace where the stacks or demos are deployed'
            cand --product-ns 'Namespace where the stacks or demos are deployed'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;stacklet;help'= {
            cand credentials 'Display credentials for a stacklet'
            cand list 'List deployed stacklets'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'stackablectl;stacklet;help;credentials'= {
        }
        &'stackablectl;stacklet;help;list'= {
        }
        &'stackablectl;stacklet;help;help'= {
        }
        &'stackablectl;demo'= {
            cand --release 'Target a specific Stackable release'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
            cand list 'List available demos'
            cand describe 'Print out detailed demo information'
            cand install 'Install a specific demo'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'stackablectl;demo;list'= {
            cand -o 'o'
            cand --output 'output'
            cand --release 'Target a specific Stackable release'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;demo;describe'= {
            cand -o 'o'
            cand --output 'output'
            cand --release 'Target a specific Stackable release'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;demo;install'= {
            cand --stack-parameters 'List of parameters to use when installing the stack'
            cand --parameters 'List of parameters to use when installing the demo'
            cand -c 'Type of local cluster to use for testing'
            cand --cluster 'Type of local cluster to use for testing'
            cand --cluster-name 'Name of the local cluster'
            cand --cluster-nodes 'Number of total nodes in the local cluster'
            cand --cluster-cp-nodes 'Number of control plane nodes in the local cluster'
            cand --operator-namespace 'Namespace where the operators are deployed'
            cand --operator-ns 'Namespace where the operators are deployed'
            cand -n 'Namespace where the stacks or demos are deployed'
            cand --namespace 'Namespace where the stacks or demos are deployed'
            cand --product-ns 'Namespace where the stacks or demos are deployed'
            cand --release 'Target a specific Stackable release'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --skip-release 'Skip the installation of the release during the stack install process'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;demo;help'= {
            cand list 'List available demos'
            cand describe 'Print out detailed demo information'
            cand install 'Install a specific demo'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'stackablectl;demo;help;list'= {
        }
        &'stackablectl;demo;help;describe'= {
        }
        &'stackablectl;demo;help;install'= {
        }
        &'stackablectl;demo;help;help'= {
        }
        &'stackablectl;completions'= {
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
            cand bash 'Generate shell completions for Bash'
            cand elvish 'Generate shell completions for Elvish'
            cand fish 'Generate shell completions for Fish'
            cand nushell 'Generate shell completions for Nushell'
            cand zsh 'Generate shell completions for ZSH'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'stackablectl;completions;bash'= {
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;completions;elvish'= {
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;completions;fish'= {
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;completions;nushell'= {
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;completions;zsh'= {
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;completions;help'= {
            cand bash 'Generate shell completions for Bash'
            cand elvish 'Generate shell completions for Elvish'
            cand fish 'Generate shell completions for Fish'
            cand nushell 'Generate shell completions for Nushell'
            cand zsh 'Generate shell completions for ZSH'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'stackablectl;completions;help;bash'= {
        }
        &'stackablectl;completions;help;elvish'= {
        }
        &'stackablectl;completions;help;fish'= {
        }
        &'stackablectl;completions;help;nushell'= {
        }
        &'stackablectl;completions;help;zsh'= {
        }
        &'stackablectl;completions;help;help'= {
        }
        &'stackablectl;cache'= {
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
            cand list 'List cached files'
            cand clean 'Clean cached files'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'stackablectl;cache;list'= {
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;cache;clean'= {
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --old 'Only remove outdated files in the cache'
            cand --outdated 'Only remove outdated files in the cache'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;cache;help'= {
            cand list 'List cached files'
            cand clean 'Clean cached files'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'stackablectl;cache;help;list'= {
        }
        &'stackablectl;cache;help;clean'= {
        }
        &'stackablectl;cache;help;help'= {
        }
        &'stackablectl;experimental-debug'= {
            cand -n 'The namespace of the Pod being debugged'
            cand --namespace 'The namespace of the Pod being debugged'
            cand -c 'The target container to debug'
            cand --container 'The target container to debug'
            cand --image 'The debug container image'
            cand -l 'Log level this application uses'
            cand --log-level 'Log level this application uses'
            cand -d 'Provide one or more additional (custom) demo file(s)'
            cand --demo-file 'Provide one or more additional (custom) demo file(s)'
            cand -s 'Provide one or more additional (custom) stack file(s)'
            cand --stack-file 'Provide one or more additional (custom) stack file(s)'
            cand -r 'Provide one or more additional (custom) release file(s)'
            cand --release-file 'Provide one or more additional (custom) release file(s)'
            cand --helm-repo-stable 'Provide a custom Helm stable repository URL'
            cand --helm-repo-test 'Provide a custom Helm test repository URL'
            cand --helm-repo-dev 'Provide a custom Helm dev repository URL'
            cand --chart-source 'Source the charts from either a OCI registry or from index.yaml-based repositories'
            cand --listener-class-preset 'Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`)'
            cand --no-cache 'Do not cache the remote (default) demo, stack and release files'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'stackablectl;help'= {
            cand operator 'Interact with single operator instead of the full platform'
            cand release 'Interact with all operators of the platform which are released together'
            cand stack 'Interact with stacks, which are ready-to-use product combinations'
            cand stacklet 'Interact with deployed stacklets, which are bundles of resources and containers required to run the product'
            cand demo 'Interact with demos, which are end-to-end usage demonstrations of the Stackable data platform'
            cand completions 'Generate shell completions for this tool'
            cand cache 'Interact with locally cached files'
            cand experimental-debug 'EXPERIMENTAL: Launch a debug container for a Pod'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'stackablectl;help;operator'= {
            cand list 'List available operators'
            cand describe 'Print out detailed operator information'
            cand install 'Install one or more operators'
            cand uninstall 'Uninstall one or more operators'
            cand installed 'List installed operators'
        }
        &'stackablectl;help;operator;list'= {
        }
        &'stackablectl;help;operator;describe'= {
        }
        &'stackablectl;help;operator;install'= {
        }
        &'stackablectl;help;operator;uninstall'= {
        }
        &'stackablectl;help;operator;installed'= {
        }
        &'stackablectl;help;release'= {
            cand list 'List available releases'
            cand describe 'Print out detailed release information'
            cand install 'Install a specific release'
            cand uninstall 'Uninstall a release'
            cand upgrade 'Upgrade a release'
        }
        &'stackablectl;help;release;list'= {
        }
        &'stackablectl;help;release;describe'= {
        }
        &'stackablectl;help;release;install'= {
        }
        &'stackablectl;help;release;uninstall'= {
        }
        &'stackablectl;help;release;upgrade'= {
        }
        &'stackablectl;help;stack'= {
            cand list 'List available stacks'
            cand describe 'Describe a specific stack'
            cand install 'Install a specific stack'
        }
        &'stackablectl;help;stack;list'= {
        }
        &'stackablectl;help;stack;describe'= {
        }
        &'stackablectl;help;stack;install'= {
        }
        &'stackablectl;help;stacklet'= {
            cand credentials 'Display credentials for a stacklet'
            cand list 'List deployed stacklets'
        }
        &'stackablectl;help;stacklet;credentials'= {
        }
        &'stackablectl;help;stacklet;list'= {
        }
        &'stackablectl;help;demo'= {
            cand list 'List available demos'
            cand describe 'Print out detailed demo information'
            cand install 'Install a specific demo'
        }
        &'stackablectl;help;demo;list'= {
        }
        &'stackablectl;help;demo;describe'= {
        }
        &'stackablectl;help;demo;install'= {
        }
        &'stackablectl;help;completions'= {
            cand bash 'Generate shell completions for Bash'
            cand elvish 'Generate shell completions for Elvish'
            cand fish 'Generate shell completions for Fish'
            cand nushell 'Generate shell completions for Nushell'
            cand zsh 'Generate shell completions for ZSH'
        }
        &'stackablectl;help;completions;bash'= {
        }
        &'stackablectl;help;completions;elvish'= {
        }
        &'stackablectl;help;completions;fish'= {
        }
        &'stackablectl;help;completions;nushell'= {
        }
        &'stackablectl;help;completions;zsh'= {
        }
        &'stackablectl;help;cache'= {
            cand list 'List cached files'
            cand clean 'Clean cached files'
        }
        &'stackablectl;help;cache;list'= {
        }
        &'stackablectl;help;cache;clean'= {
        }
        &'stackablectl;help;experimental-debug'= {
        }
        &'stackablectl;help;help'= {
        }
    ]
    $completions[$command]
}
