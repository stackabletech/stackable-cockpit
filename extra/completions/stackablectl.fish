# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_stackablectl_global_optspecs
	string join \n l/log-level= no-cache offline d/demo-file= s/stack-file= r/release-file= helm-repo-stable= helm-repo-test= helm-repo-dev= h/help V/version
end

function __fish_stackablectl_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_stackablectl_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_stackablectl_using_subcommand
	set -l cmd (__fish_stackablectl_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c stackablectl -n "__fish_stackablectl_needs_command" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_needs_command" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_needs_command" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_needs_command" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_needs_command" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_needs_command" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_needs_command" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_needs_command" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_needs_command" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_needs_command" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_needs_command" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_needs_command" -f -a "operator" -d 'Interact with single operator instead of the full platform'
complete -c stackablectl -n "__fish_stackablectl_needs_command" -f -a "release" -d 'Interact with all operators of the platform which are released together'
complete -c stackablectl -n "__fish_stackablectl_needs_command" -f -a "stack" -d 'Interact with stacks, which are ready-to-use product combinations'
complete -c stackablectl -n "__fish_stackablectl_needs_command" -f -a "stacklet" -d 'Interact with deployed stacklets, which are bundles of resources and containers required to run the product'
complete -c stackablectl -n "__fish_stackablectl_needs_command" -f -a "demo" -d 'Interact with demos, which are end-to-end usage demonstrations of the Stackable data platform'
complete -c stackablectl -n "__fish_stackablectl_needs_command" -f -a "completions" -d 'Generate shell completions for this tool'
complete -c stackablectl -n "__fish_stackablectl_needs_command" -f -a "cache" -d 'Interact with locally cached files'
complete -c stackablectl -n "__fish_stackablectl_needs_command" -f -a "experimental-debug" -d 'EXPERIMENTAL: Launch a debug container for a Pod'
complete -c stackablectl -n "__fish_stackablectl_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and not __fish_seen_subcommand_from list describe install uninstall installed help" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and not __fish_seen_subcommand_from list describe install uninstall installed help" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and not __fish_seen_subcommand_from list describe install uninstall installed help" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and not __fish_seen_subcommand_from list describe install uninstall installed help" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and not __fish_seen_subcommand_from list describe install uninstall installed help" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and not __fish_seen_subcommand_from list describe install uninstall installed help" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and not __fish_seen_subcommand_from list describe install uninstall installed help" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and not __fish_seen_subcommand_from list describe install uninstall installed help" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and not __fish_seen_subcommand_from list describe install uninstall installed help" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and not __fish_seen_subcommand_from list describe install uninstall installed help" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and not __fish_seen_subcommand_from list describe install uninstall installed help" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and not __fish_seen_subcommand_from list describe install uninstall installed help" -f -a "list" -d 'List available operators'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and not __fish_seen_subcommand_from list describe install uninstall installed help" -f -a "describe" -d 'Print out detailed operator information'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and not __fish_seen_subcommand_from list describe install uninstall installed help" -f -a "install" -d 'Install one or more operators'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and not __fish_seen_subcommand_from list describe install uninstall installed help" -f -a "uninstall" -d 'Uninstall one or more operators'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and not __fish_seen_subcommand_from list describe install uninstall installed help" -f -a "installed" -d 'List installed operators'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and not __fish_seen_subcommand_from list describe install uninstall installed help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from list" -s o -l output -r -f -a "{plain\t'Print output formatted as plain text',table\t'Print output formatted as a table',json\t'Print output formatted as JSON',yaml\t'Print output formatted as YAML'}"
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from list" -l chart-source -r -f -a "{oci\t'OCI registry',repo\t'Nexus repositories: resolution (dev, test, stable) is based on the version and thus may be operator-specific',tgz\t'Archive'}"
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from list" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from list" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from list" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from list" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from list" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from list" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from list" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from list" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from list" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from list" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from describe" -s o -l output -r -f -a "{plain\t'Print output formatted as plain text',table\t'Print output formatted as a table',json\t'Print output formatted as JSON',yaml\t'Print output formatted as YAML'}"
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from describe" -l chart-source -r -f -a "{oci\t'OCI registry',repo\t'Nexus repositories: resolution (dev, test, stable) is based on the version and thus may be operator-specific',tgz\t'Archive'}"
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from describe" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from describe" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from describe" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from describe" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from describe" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from describe" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from describe" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from describe" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from describe" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from describe" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from describe" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from install" -l operator-namespace -l operator-ns -d 'Namespace in the cluster used to deploy the operators' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from install" -s c -l cluster -d 'Type of local cluster to use for testing' -r -f -a "{kind\t'Use a kind cluster, see <https://docs.stackable.tech/home/getting_started.html#_installing_kubernetes_using_kind>',minikube\t'Use a minikube cluster'}"
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from install" -l cluster-name -d 'Name of the local cluster' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from install" -l cluster-nodes -d 'Number of total nodes in the local cluster' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from install" -l cluster-cp-nodes -d 'Number of control plane nodes in the local cluster' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from install" -l chart-source -r -f -a "{oci\t'OCI registry',repo\t'Nexus repositories: resolution (dev, test, stable) is based on the version and thus may be operator-specific',tgz\t'Archive'}"
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from install" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from install" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from install" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from install" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from install" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from install" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from install" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from install" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from install" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from install" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from install" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from uninstall" -l operator-namespace -l operator-ns -d 'Namespace in the cluster used to deploy the operators' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from uninstall" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from uninstall" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from uninstall" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from uninstall" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from uninstall" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from uninstall" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from uninstall" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from uninstall" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from uninstall" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from uninstall" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from uninstall" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from installed" -s o -l output -r -f -a "{plain\t'Print output formatted as plain text',table\t'Print output formatted as a table',json\t'Print output formatted as JSON',yaml\t'Print output formatted as YAML'}"
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from installed" -l operator-namespace -l operator-ns -d 'Namespace in the cluster used to deploy the operators' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from installed" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from installed" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from installed" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from installed" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from installed" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from installed" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from installed" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from installed" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from installed" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from installed" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from installed" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from help" -f -a "list" -d 'List available operators'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from help" -f -a "describe" -d 'Print out detailed operator information'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from help" -f -a "install" -d 'Install one or more operators'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from help" -f -a "uninstall" -d 'Uninstall one or more operators'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from help" -f -a "installed" -d 'List installed operators'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand operator; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and not __fish_seen_subcommand_from list describe install uninstall help" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and not __fish_seen_subcommand_from list describe install uninstall help" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and not __fish_seen_subcommand_from list describe install uninstall help" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and not __fish_seen_subcommand_from list describe install uninstall help" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and not __fish_seen_subcommand_from list describe install uninstall help" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and not __fish_seen_subcommand_from list describe install uninstall help" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and not __fish_seen_subcommand_from list describe install uninstall help" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and not __fish_seen_subcommand_from list describe install uninstall help" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and not __fish_seen_subcommand_from list describe install uninstall help" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and not __fish_seen_subcommand_from list describe install uninstall help" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and not __fish_seen_subcommand_from list describe install uninstall help" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and not __fish_seen_subcommand_from list describe install uninstall help" -f -a "list" -d 'List available releases'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and not __fish_seen_subcommand_from list describe install uninstall help" -f -a "describe" -d 'Print out detailed release information'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and not __fish_seen_subcommand_from list describe install uninstall help" -f -a "install" -d 'Install a specific release'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and not __fish_seen_subcommand_from list describe install uninstall help" -f -a "uninstall" -d 'Uninstall a release'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and not __fish_seen_subcommand_from list describe install uninstall help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from list" -s o -l output -r -f -a "{plain\t'Print output formatted as plain text',table\t'Print output formatted as a table',json\t'Print output formatted as JSON',yaml\t'Print output formatted as YAML'}"
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from list" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from list" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from list" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from list" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from list" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from list" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from list" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from list" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from list" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from list" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from describe" -s o -l output -r -f -a "{plain\t'Print output formatted as plain text',table\t'Print output formatted as a table',json\t'Print output formatted as JSON',yaml\t'Print output formatted as YAML'}"
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from describe" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from describe" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from describe" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from describe" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from describe" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from describe" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from describe" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from describe" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from describe" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from describe" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from describe" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from install" -s i -l include -d 'Whitelist of product operators to install' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from install" -s e -l exclude -d 'Blacklist of product operators to install' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from install" -l operator-namespace -l operator-ns -d 'Namespace in the cluster used to deploy the operators' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from install" -s c -l cluster -d 'Type of local cluster to use for testing' -r -f -a "{kind\t'Use a kind cluster, see <https://docs.stackable.tech/home/getting_started.html#_installing_kubernetes_using_kind>',minikube\t'Use a minikube cluster'}"
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from install" -l cluster-name -d 'Name of the local cluster' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from install" -l cluster-nodes -d 'Number of total nodes in the local cluster' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from install" -l cluster-cp-nodes -d 'Number of control plane nodes in the local cluster' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from install" -l chart-source -r -f -a "{oci\t'OCI registry',repo\t'Nexus repositories: resolution (dev, test, stable) is based on the version and thus may be operator-specific',tgz\t'Archive'}"
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from install" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from install" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from install" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from install" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from install" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from install" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from install" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from install" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from install" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from install" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from install" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from uninstall" -l operator-namespace -l operator-ns -d 'Namespace in the cluster used to deploy the operators' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from uninstall" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from uninstall" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from uninstall" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from uninstall" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from uninstall" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from uninstall" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from uninstall" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from uninstall" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from uninstall" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from uninstall" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from uninstall" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from help" -f -a "list" -d 'List available releases'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from help" -f -a "describe" -d 'Print out detailed release information'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from help" -f -a "install" -d 'Install a specific release'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from help" -f -a "uninstall" -d 'Uninstall a release'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand release; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and not __fish_seen_subcommand_from list describe install help" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and not __fish_seen_subcommand_from list describe install help" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and not __fish_seen_subcommand_from list describe install help" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and not __fish_seen_subcommand_from list describe install help" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and not __fish_seen_subcommand_from list describe install help" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and not __fish_seen_subcommand_from list describe install help" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and not __fish_seen_subcommand_from list describe install help" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and not __fish_seen_subcommand_from list describe install help" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and not __fish_seen_subcommand_from list describe install help" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and not __fish_seen_subcommand_from list describe install help" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and not __fish_seen_subcommand_from list describe install help" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and not __fish_seen_subcommand_from list describe install help" -f -a "list" -d 'List available stacks'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and not __fish_seen_subcommand_from list describe install help" -f -a "describe" -d 'Describe a specific stack'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and not __fish_seen_subcommand_from list describe install help" -f -a "install" -d 'Install a specific stack'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and not __fish_seen_subcommand_from list describe install help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from list" -s o -l output -r -f -a "{plain\t'Print output formatted as plain text',table\t'Print output formatted as a table',json\t'Print output formatted as JSON',yaml\t'Print output formatted as YAML'}"
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from list" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from list" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from list" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from list" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from list" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from list" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from list" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from list" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from list" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from list" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from describe" -s o -l output -r -f -a "{plain\t'Print output formatted as plain text',table\t'Print output formatted as a table',json\t'Print output formatted as JSON',yaml\t'Print output formatted as YAML'}"
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from describe" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from describe" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from describe" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from describe" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from describe" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from describe" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from describe" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from describe" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from describe" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from describe" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from describe" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -l stack-parameters -d 'List of parameters to use when installing the stack' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -l parameters -d 'List of parameters to use when installing the stack' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -s c -l cluster -d 'Type of local cluster to use for testing' -r -f -a "{kind\t'Use a kind cluster, see <https://docs.stackable.tech/home/getting_started.html#_installing_kubernetes_using_kind>',minikube\t'Use a minikube cluster'}"
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -l cluster-name -d 'Name of the local cluster' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -l cluster-nodes -d 'Number of total nodes in the local cluster' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -l cluster-cp-nodes -d 'Number of control plane nodes in the local cluster' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -l operator-namespace -l operator-ns -d 'Namespace where the operators are deployed' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -s n -l product-namespace -l product-ns -d 'Namespace where the products (e.g. stacks or demos) are deployed' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -l chart-source -r -f -a "{oci\t'OCI registry',repo\t'Nexus repositories: resolution (dev, test, stable) is based on the version and thus may be operator-specific',tgz\t'Archive'}"
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -l skip-release -d 'Skip the installation of the release during the stack install process'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from install" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from help" -f -a "list" -d 'List available stacks'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from help" -f -a "describe" -d 'Describe a specific stack'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from help" -f -a "install" -d 'Install a specific stack'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stack; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and not __fish_seen_subcommand_from credentials list help" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and not __fish_seen_subcommand_from credentials list help" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and not __fish_seen_subcommand_from credentials list help" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and not __fish_seen_subcommand_from credentials list help" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and not __fish_seen_subcommand_from credentials list help" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and not __fish_seen_subcommand_from credentials list help" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and not __fish_seen_subcommand_from credentials list help" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and not __fish_seen_subcommand_from credentials list help" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and not __fish_seen_subcommand_from credentials list help" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and not __fish_seen_subcommand_from credentials list help" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and not __fish_seen_subcommand_from credentials list help" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and not __fish_seen_subcommand_from credentials list help" -f -a "credentials" -d 'Display credentials for a stacklet'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and not __fish_seen_subcommand_from credentials list help" -f -a "list" -d 'List deployed stacklets'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and not __fish_seen_subcommand_from credentials list help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from credentials" -s n -l product-namespace -l product-ns -d 'Namespace in the cluster used to deploy the products' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from credentials" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from credentials" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from credentials" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from credentials" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from credentials" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from credentials" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from credentials" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from credentials" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from credentials" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from credentials" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from credentials" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from list" -s o -l output -r -f -a "{plain\t'Print output formatted as plain text',table\t'Print output formatted as a table',json\t'Print output formatted as JSON',yaml\t'Print output formatted as YAML'}"
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from list" -l operator-namespace -l operator-ns -d 'Namespace where the operators are deployed' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from list" -s n -l product-namespace -l product-ns -d 'Namespace where the products (e.g. stacks or demos) are deployed' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from list" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from list" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from list" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from list" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from list" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from list" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from list" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from list" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from list" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from list" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from help" -f -a "credentials" -d 'Display credentials for a stacklet'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from help" -f -a "list" -d 'List deployed stacklets'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand stacklet; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and not __fish_seen_subcommand_from list describe install help" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and not __fish_seen_subcommand_from list describe install help" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and not __fish_seen_subcommand_from list describe install help" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and not __fish_seen_subcommand_from list describe install help" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and not __fish_seen_subcommand_from list describe install help" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and not __fish_seen_subcommand_from list describe install help" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and not __fish_seen_subcommand_from list describe install help" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and not __fish_seen_subcommand_from list describe install help" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and not __fish_seen_subcommand_from list describe install help" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and not __fish_seen_subcommand_from list describe install help" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and not __fish_seen_subcommand_from list describe install help" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and not __fish_seen_subcommand_from list describe install help" -f -a "list" -d 'List available demos'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and not __fish_seen_subcommand_from list describe install help" -f -a "describe" -d 'Print out detailed demo information'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and not __fish_seen_subcommand_from list describe install help" -f -a "install" -d 'Install a specific demo'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and not __fish_seen_subcommand_from list describe install help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from list" -s o -l output -r -f -a "{plain\t'Print output formatted as plain text',table\t'Print output formatted as a table',json\t'Print output formatted as JSON',yaml\t'Print output formatted as YAML'}"
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from list" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from list" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from list" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from list" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from list" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from list" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from list" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from list" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from list" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from list" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from describe" -s o -l output -r -f -a "{plain\t'Print output formatted as plain text',table\t'Print output formatted as a table',json\t'Print output formatted as JSON',yaml\t'Print output formatted as YAML'}"
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from describe" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from describe" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from describe" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from describe" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from describe" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from describe" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from describe" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from describe" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from describe" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from describe" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from describe" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -l stack-parameters -d 'List of parameters to use when installing the stack' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -l parameters -d 'List of parameters to use when installing the demo' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -s c -l cluster -d 'Type of local cluster to use for testing' -r -f -a "{kind\t'Use a kind cluster, see <https://docs.stackable.tech/home/getting_started.html#_installing_kubernetes_using_kind>',minikube\t'Use a minikube cluster'}"
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -l cluster-name -d 'Name of the local cluster' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -l cluster-nodes -d 'Number of total nodes in the local cluster' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -l cluster-cp-nodes -d 'Number of control plane nodes in the local cluster' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -l operator-namespace -l operator-ns -d 'Namespace where the operators are deployed' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -s n -l product-namespace -l product-ns -d 'Namespace where the products (e.g. stacks or demos) are deployed' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -l chart-source -r -f -a "{oci\t'OCI registry',repo\t'Nexus repositories: resolution (dev, test, stable) is based on the version and thus may be operator-specific',tgz\t'Archive'}"
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -l skip-release -d 'Skip the installation of the release during the stack install process'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from install" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from help" -f -a "list" -d 'List available demos'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from help" -f -a "describe" -d 'Print out detailed demo information'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from help" -f -a "install" -d 'Install a specific demo'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand demo; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and not __fish_seen_subcommand_from bash elvish fish nushell zsh help" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and not __fish_seen_subcommand_from bash elvish fish nushell zsh help" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and not __fish_seen_subcommand_from bash elvish fish nushell zsh help" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and not __fish_seen_subcommand_from bash elvish fish nushell zsh help" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and not __fish_seen_subcommand_from bash elvish fish nushell zsh help" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and not __fish_seen_subcommand_from bash elvish fish nushell zsh help" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and not __fish_seen_subcommand_from bash elvish fish nushell zsh help" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and not __fish_seen_subcommand_from bash elvish fish nushell zsh help" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and not __fish_seen_subcommand_from bash elvish fish nushell zsh help" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and not __fish_seen_subcommand_from bash elvish fish nushell zsh help" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and not __fish_seen_subcommand_from bash elvish fish nushell zsh help" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and not __fish_seen_subcommand_from bash elvish fish nushell zsh help" -f -a "bash" -d 'Generate shell completions for Bash'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and not __fish_seen_subcommand_from bash elvish fish nushell zsh help" -f -a "elvish" -d 'Generate shell completions for Elvish'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and not __fish_seen_subcommand_from bash elvish fish nushell zsh help" -f -a "fish" -d 'Generate shell completions for Fish'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and not __fish_seen_subcommand_from bash elvish fish nushell zsh help" -f -a "nushell" -d 'Generate shell completions for Nushell'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and not __fish_seen_subcommand_from bash elvish fish nushell zsh help" -f -a "zsh" -d 'Generate shell completions for ZSH'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and not __fish_seen_subcommand_from bash elvish fish nushell zsh help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from bash" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from bash" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from bash" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from bash" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from bash" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from bash" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from bash" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from bash" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from bash" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from bash" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from bash" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from elvish" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from elvish" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from elvish" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from elvish" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from elvish" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from elvish" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from elvish" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from elvish" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from elvish" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from elvish" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from elvish" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from fish" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from fish" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from fish" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from fish" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from fish" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from fish" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from fish" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from fish" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from fish" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from fish" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from fish" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from nushell" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from nushell" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from nushell" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from nushell" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from nushell" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from nushell" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from nushell" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from nushell" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from nushell" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from nushell" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from nushell" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from zsh" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from zsh" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from zsh" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from zsh" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from zsh" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from zsh" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from zsh" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from zsh" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from zsh" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from zsh" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from zsh" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from help" -f -a "bash" -d 'Generate shell completions for Bash'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from help" -f -a "elvish" -d 'Generate shell completions for Elvish'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from help" -f -a "fish" -d 'Generate shell completions for Fish'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from help" -f -a "nushell" -d 'Generate shell completions for Nushell'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from help" -f -a "zsh" -d 'Generate shell completions for ZSH'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand completions; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and not __fish_seen_subcommand_from list clean help" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and not __fish_seen_subcommand_from list clean help" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and not __fish_seen_subcommand_from list clean help" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and not __fish_seen_subcommand_from list clean help" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and not __fish_seen_subcommand_from list clean help" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and not __fish_seen_subcommand_from list clean help" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and not __fish_seen_subcommand_from list clean help" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and not __fish_seen_subcommand_from list clean help" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and not __fish_seen_subcommand_from list clean help" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and not __fish_seen_subcommand_from list clean help" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and not __fish_seen_subcommand_from list clean help" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and not __fish_seen_subcommand_from list clean help" -f -a "list" -d 'List cached files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and not __fish_seen_subcommand_from list clean help" -f -a "clean" -d 'Clean cached files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and not __fish_seen_subcommand_from list clean help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from list" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from list" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from list" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from list" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from list" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from list" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from list" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from list" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from list" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from list" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from clean" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from clean" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from clean" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from clean" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from clean" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from clean" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from clean" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from clean" -l old -l outdated -d 'Only remove outdated files in the cache'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from clean" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from clean" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from clean" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from clean" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from help" -f -a "list" -d 'List cached files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from help" -f -a "clean" -d 'Clean cached files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand cache; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand experimental-debug" -s n -l namespace -d 'The namespace of the Pod being debugged' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand experimental-debug" -s c -l container -d 'The target container to debug' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand experimental-debug" -l image -d 'The debug container image' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand experimental-debug" -s l -l log-level -d 'Log level this application uses' -r
complete -c stackablectl -n "__fish_stackablectl_using_subcommand experimental-debug" -s d -l demo-file -d 'Provide one or more additional (custom) demo file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand experimental-debug" -s s -l stack-file -d 'Provide one or more additional (custom) stack file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand experimental-debug" -s r -l release-file -d 'Provide one or more additional (custom) release file(s)' -r -F
complete -c stackablectl -n "__fish_stackablectl_using_subcommand experimental-debug" -l helm-repo-stable -d 'Provide a custom Helm stable repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand experimental-debug" -l helm-repo-test -d 'Provide a custom Helm test repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand experimental-debug" -l helm-repo-dev -d 'Provide a custom Helm dev repository URL' -r -f
complete -c stackablectl -n "__fish_stackablectl_using_subcommand experimental-debug" -l no-cache -d 'Do not cache the remote (default) demo, stack and release files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand experimental-debug" -l offline -d 'Do not request any remote files via the network'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand experimental-debug" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand experimental-debug" -s V -l version -d 'Print version'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and not __fish_seen_subcommand_from operator release stack stacklet demo completions cache experimental-debug help" -f -a "operator" -d 'Interact with single operator instead of the full platform'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and not __fish_seen_subcommand_from operator release stack stacklet demo completions cache experimental-debug help" -f -a "release" -d 'Interact with all operators of the platform which are released together'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and not __fish_seen_subcommand_from operator release stack stacklet demo completions cache experimental-debug help" -f -a "stack" -d 'Interact with stacks, which are ready-to-use product combinations'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and not __fish_seen_subcommand_from operator release stack stacklet demo completions cache experimental-debug help" -f -a "stacklet" -d 'Interact with deployed stacklets, which are bundles of resources and containers required to run the product'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and not __fish_seen_subcommand_from operator release stack stacklet demo completions cache experimental-debug help" -f -a "demo" -d 'Interact with demos, which are end-to-end usage demonstrations of the Stackable data platform'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and not __fish_seen_subcommand_from operator release stack stacklet demo completions cache experimental-debug help" -f -a "completions" -d 'Generate shell completions for this tool'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and not __fish_seen_subcommand_from operator release stack stacklet demo completions cache experimental-debug help" -f -a "cache" -d 'Interact with locally cached files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and not __fish_seen_subcommand_from operator release stack stacklet demo completions cache experimental-debug help" -f -a "experimental-debug" -d 'EXPERIMENTAL: Launch a debug container for a Pod'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and not __fish_seen_subcommand_from operator release stack stacklet demo completions cache experimental-debug help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from operator" -f -a "list" -d 'List available operators'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from operator" -f -a "describe" -d 'Print out detailed operator information'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from operator" -f -a "install" -d 'Install one or more operators'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from operator" -f -a "uninstall" -d 'Uninstall one or more operators'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from operator" -f -a "installed" -d 'List installed operators'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from release" -f -a "list" -d 'List available releases'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from release" -f -a "describe" -d 'Print out detailed release information'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from release" -f -a "install" -d 'Install a specific release'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from release" -f -a "uninstall" -d 'Uninstall a release'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from stack" -f -a "list" -d 'List available stacks'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from stack" -f -a "describe" -d 'Describe a specific stack'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from stack" -f -a "install" -d 'Install a specific stack'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from stacklet" -f -a "credentials" -d 'Display credentials for a stacklet'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from stacklet" -f -a "list" -d 'List deployed stacklets'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from demo" -f -a "list" -d 'List available demos'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from demo" -f -a "describe" -d 'Print out detailed demo information'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from demo" -f -a "install" -d 'Install a specific demo'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from completions" -f -a "bash" -d 'Generate shell completions for Bash'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from completions" -f -a "elvish" -d 'Generate shell completions for Elvish'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from completions" -f -a "fish" -d 'Generate shell completions for Fish'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from completions" -f -a "nushell" -d 'Generate shell completions for Nushell'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from completions" -f -a "zsh" -d 'Generate shell completions for ZSH'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from cache" -f -a "list" -d 'List cached files'
complete -c stackablectl -n "__fish_stackablectl_using_subcommand help; and __fish_seen_subcommand_from cache" -f -a "clean" -d 'Clean cached files'
