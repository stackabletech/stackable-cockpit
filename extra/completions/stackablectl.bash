_stackablectl() {
    local i cur prev opts cmd
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="stackablectl"
                ;;
            stackablectl,cache)
                cmd="stackablectl__cache"
                ;;
            stackablectl,completions)
                cmd="stackablectl__completions"
                ;;
            stackablectl,demo)
                cmd="stackablectl__demo"
                ;;
            stackablectl,help)
                cmd="stackablectl__help"
                ;;
            stackablectl,operator)
                cmd="stackablectl__operator"
                ;;
            stackablectl,release)
                cmd="stackablectl__release"
                ;;
            stackablectl,stack)
                cmd="stackablectl__stack"
                ;;
            stackablectl,stacklet)
                cmd="stackablectl__stacklet"
                ;;
            stackablectl__cache,clean)
                cmd="stackablectl__cache__clean"
                ;;
            stackablectl__cache,help)
                cmd="stackablectl__cache__help"
                ;;
            stackablectl__cache,list)
                cmd="stackablectl__cache__list"
                ;;
            stackablectl__cache__help,clean)
                cmd="stackablectl__cache__help__clean"
                ;;
            stackablectl__cache__help,help)
                cmd="stackablectl__cache__help__help"
                ;;
            stackablectl__cache__help,list)
                cmd="stackablectl__cache__help__list"
                ;;
            stackablectl__completions,bash)
                cmd="stackablectl__completions__bash"
                ;;
            stackablectl__completions,fish)
                cmd="stackablectl__completions__fish"
                ;;
            stackablectl__completions,help)
                cmd="stackablectl__completions__help"
                ;;
            stackablectl__completions,zsh)
                cmd="stackablectl__completions__zsh"
                ;;
            stackablectl__completions__help,bash)
                cmd="stackablectl__completions__help__bash"
                ;;
            stackablectl__completions__help,fish)
                cmd="stackablectl__completions__help__fish"
                ;;
            stackablectl__completions__help,help)
                cmd="stackablectl__completions__help__help"
                ;;
            stackablectl__completions__help,zsh)
                cmd="stackablectl__completions__help__zsh"
                ;;
            stackablectl__demo,describe)
                cmd="stackablectl__demo__describe"
                ;;
            stackablectl__demo,help)
                cmd="stackablectl__demo__help"
                ;;
            stackablectl__demo,install)
                cmd="stackablectl__demo__install"
                ;;
            stackablectl__demo,list)
                cmd="stackablectl__demo__list"
                ;;
            stackablectl__demo__help,describe)
                cmd="stackablectl__demo__help__describe"
                ;;
            stackablectl__demo__help,help)
                cmd="stackablectl__demo__help__help"
                ;;
            stackablectl__demo__help,install)
                cmd="stackablectl__demo__help__install"
                ;;
            stackablectl__demo__help,list)
                cmd="stackablectl__demo__help__list"
                ;;
            stackablectl__help,cache)
                cmd="stackablectl__help__cache"
                ;;
            stackablectl__help,completions)
                cmd="stackablectl__help__completions"
                ;;
            stackablectl__help,demo)
                cmd="stackablectl__help__demo"
                ;;
            stackablectl__help,help)
                cmd="stackablectl__help__help"
                ;;
            stackablectl__help,operator)
                cmd="stackablectl__help__operator"
                ;;
            stackablectl__help,release)
                cmd="stackablectl__help__release"
                ;;
            stackablectl__help,stack)
                cmd="stackablectl__help__stack"
                ;;
            stackablectl__help,stacklet)
                cmd="stackablectl__help__stacklet"
                ;;
            stackablectl__help__cache,clean)
                cmd="stackablectl__help__cache__clean"
                ;;
            stackablectl__help__cache,list)
                cmd="stackablectl__help__cache__list"
                ;;
            stackablectl__help__completions,bash)
                cmd="stackablectl__help__completions__bash"
                ;;
            stackablectl__help__completions,fish)
                cmd="stackablectl__help__completions__fish"
                ;;
            stackablectl__help__completions,zsh)
                cmd="stackablectl__help__completions__zsh"
                ;;
            stackablectl__help__demo,describe)
                cmd="stackablectl__help__demo__describe"
                ;;
            stackablectl__help__demo,install)
                cmd="stackablectl__help__demo__install"
                ;;
            stackablectl__help__demo,list)
                cmd="stackablectl__help__demo__list"
                ;;
            stackablectl__help__operator,describe)
                cmd="stackablectl__help__operator__describe"
                ;;
            stackablectl__help__operator,install)
                cmd="stackablectl__help__operator__install"
                ;;
            stackablectl__help__operator,installed)
                cmd="stackablectl__help__operator__installed"
                ;;
            stackablectl__help__operator,list)
                cmd="stackablectl__help__operator__list"
                ;;
            stackablectl__help__operator,uninstall)
                cmd="stackablectl__help__operator__uninstall"
                ;;
            stackablectl__help__release,describe)
                cmd="stackablectl__help__release__describe"
                ;;
            stackablectl__help__release,install)
                cmd="stackablectl__help__release__install"
                ;;
            stackablectl__help__release,list)
                cmd="stackablectl__help__release__list"
                ;;
            stackablectl__help__release,uninstall)
                cmd="stackablectl__help__release__uninstall"
                ;;
            stackablectl__help__stack,describe)
                cmd="stackablectl__help__stack__describe"
                ;;
            stackablectl__help__stack,install)
                cmd="stackablectl__help__stack__install"
                ;;
            stackablectl__help__stack,list)
                cmd="stackablectl__help__stack__list"
                ;;
            stackablectl__help__stacklet,credentials)
                cmd="stackablectl__help__stacklet__credentials"
                ;;
            stackablectl__help__stacklet,list)
                cmd="stackablectl__help__stacklet__list"
                ;;
            stackablectl__operator,describe)
                cmd="stackablectl__operator__describe"
                ;;
            stackablectl__operator,help)
                cmd="stackablectl__operator__help"
                ;;
            stackablectl__operator,install)
                cmd="stackablectl__operator__install"
                ;;
            stackablectl__operator,installed)
                cmd="stackablectl__operator__installed"
                ;;
            stackablectl__operator,list)
                cmd="stackablectl__operator__list"
                ;;
            stackablectl__operator,uninstall)
                cmd="stackablectl__operator__uninstall"
                ;;
            stackablectl__operator__help,describe)
                cmd="stackablectl__operator__help__describe"
                ;;
            stackablectl__operator__help,help)
                cmd="stackablectl__operator__help__help"
                ;;
            stackablectl__operator__help,install)
                cmd="stackablectl__operator__help__install"
                ;;
            stackablectl__operator__help,installed)
                cmd="stackablectl__operator__help__installed"
                ;;
            stackablectl__operator__help,list)
                cmd="stackablectl__operator__help__list"
                ;;
            stackablectl__operator__help,uninstall)
                cmd="stackablectl__operator__help__uninstall"
                ;;
            stackablectl__release,describe)
                cmd="stackablectl__release__describe"
                ;;
            stackablectl__release,help)
                cmd="stackablectl__release__help"
                ;;
            stackablectl__release,install)
                cmd="stackablectl__release__install"
                ;;
            stackablectl__release,list)
                cmd="stackablectl__release__list"
                ;;
            stackablectl__release,uninstall)
                cmd="stackablectl__release__uninstall"
                ;;
            stackablectl__release__help,describe)
                cmd="stackablectl__release__help__describe"
                ;;
            stackablectl__release__help,help)
                cmd="stackablectl__release__help__help"
                ;;
            stackablectl__release__help,install)
                cmd="stackablectl__release__help__install"
                ;;
            stackablectl__release__help,list)
                cmd="stackablectl__release__help__list"
                ;;
            stackablectl__release__help,uninstall)
                cmd="stackablectl__release__help__uninstall"
                ;;
            stackablectl__stack,describe)
                cmd="stackablectl__stack__describe"
                ;;
            stackablectl__stack,help)
                cmd="stackablectl__stack__help"
                ;;
            stackablectl__stack,install)
                cmd="stackablectl__stack__install"
                ;;
            stackablectl__stack,list)
                cmd="stackablectl__stack__list"
                ;;
            stackablectl__stack__help,describe)
                cmd="stackablectl__stack__help__describe"
                ;;
            stackablectl__stack__help,help)
                cmd="stackablectl__stack__help__help"
                ;;
            stackablectl__stack__help,install)
                cmd="stackablectl__stack__help__install"
                ;;
            stackablectl__stack__help,list)
                cmd="stackablectl__stack__help__list"
                ;;
            stackablectl__stacklet,credentials)
                cmd="stackablectl__stacklet__credentials"
                ;;
            stackablectl__stacklet,help)
                cmd="stackablectl__stacklet__help"
                ;;
            stackablectl__stacklet,list)
                cmd="stackablectl__stacklet__list"
                ;;
            stackablectl__stacklet__help,credentials)
                cmd="stackablectl__stacklet__help__credentials"
                ;;
            stackablectl__stacklet__help,help)
                cmd="stackablectl__stacklet__help__help"
                ;;
            stackablectl__stacklet__help,list)
                cmd="stackablectl__stacklet__help__list"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        stackablectl)
            opts="-l -d -s -r -h -V --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version operator release stack stacklet demo completions cache help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__cache)
            opts="-l -d -s -r -h -V --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version list clean help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__cache__clean)
            opts="-l -d -s -r -h -V --outdated --old --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__cache__help)
            opts="list clean help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__cache__help__clean)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__cache__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__cache__help__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__cache__list)
            opts="-l -d -s -r -h -V --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__completions)
            opts="-l -d -s -r -h -V --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version bash fish zsh help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__completions__bash)
            opts="-l -d -s -r -h -V --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__completions__fish)
            opts="-l -d -s -r -h -V --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__completions__help)
            opts="bash fish zsh help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__completions__help__bash)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__completions__help__fish)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__completions__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__completions__help__zsh)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__completions__zsh)
            opts="-l -d -s -r -h -V --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__demo)
            opts="-l -d -s -r -h -V --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version list describe install help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__demo__describe)
            opts="-o -l -d -s -r -h -V --output --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version <DEMO>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain json yaml" -- "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -W "plain json yaml" -- "${cur}"))
                    return 0
                    ;;
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__demo__help)
            opts="list describe install help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__demo__help__describe)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__demo__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__demo__help__install)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__demo__help__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__demo__install)
            opts="-c -n -l -d -s -r -h -V --skip-release --stack-parameters --parameters --cluster --cluster-name --cluster-nodes --cluster-cp-nodes --operator-ns --operator-namespace --product-ns --product-namespace --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version <DEMO>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --stack-parameters)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --parameters)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cluster)
                    COMPREPLY=($(compgen -W "kind minikube" -- "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -W "kind minikube" -- "${cur}"))
                    return 0
                    ;;
                --cluster-name)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cluster-nodes)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cluster-cp-nodes)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --operator-namespace)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --operator-ns)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --product-namespace)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --product-ns)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -n)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__demo__list)
            opts="-o -l -d -s -r -h -V --output --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain json yaml" -- "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -W "plain json yaml" -- "${cur}"))
                    return 0
                    ;;
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help)
            opts="operator release stack stacklet demo completions cache help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__cache)
            opts="list clean"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__cache__clean)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__cache__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__completions)
            opts="bash fish zsh"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__completions__bash)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__completions__fish)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__completions__zsh)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__demo)
            opts="list describe install"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__demo__describe)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__demo__install)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__demo__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__operator)
            opts="list describe install uninstall installed"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__operator__describe)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__operator__install)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__operator__installed)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__operator__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__operator__uninstall)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__release)
            opts="list describe install uninstall"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__release__describe)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__release__install)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__release__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__release__uninstall)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__stack)
            opts="list describe install"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__stack__describe)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__stack__install)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__stack__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__stacklet)
            opts="credentials list"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__stacklet__credentials)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__help__stacklet__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__operator)
            opts="-l -d -s -r -h -V --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version list describe install uninstall installed help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__operator__describe)
            opts="-o -l -d -s -r -h -V --output --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version <OPERATOR>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain json yaml" -- "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -W "plain json yaml" -- "${cur}"))
                    return 0
                    ;;
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__operator__help)
            opts="list describe install uninstall installed help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__operator__help__describe)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__operator__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__operator__help__install)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__operator__help__installed)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__operator__help__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__operator__help__uninstall)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__operator__install)
            opts="-c -l -d -s -r -h -V --operator-ns --operator-namespace --cluster --cluster-name --cluster-nodes --cluster-cp-nodes --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version <OPERATORS>..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --operator-namespace)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --operator-ns)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cluster)
                    COMPREPLY=($(compgen -W "kind minikube" -- "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -W "kind minikube" -- "${cur}"))
                    return 0
                    ;;
                --cluster-name)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cluster-nodes)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cluster-cp-nodes)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__operator__installed)
            opts="-o -l -d -s -r -h -V --output --operator-ns --operator-namespace --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain json yaml" -- "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -W "plain json yaml" -- "${cur}"))
                    return 0
                    ;;
                --operator-namespace)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --operator-ns)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__operator__list)
            opts="-o -l -d -s -r -h -V --output --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain json yaml" -- "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -W "plain json yaml" -- "${cur}"))
                    return 0
                    ;;
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__operator__uninstall)
            opts="-l -d -s -r -h -V --operator-ns --operator-namespace --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version <OPERATORS>..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --operator-namespace)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --operator-ns)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__release)
            opts="-l -d -s -r -h -V --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version list describe install uninstall help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__release__describe)
            opts="-o -l -d -s -r -h -V --output --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version <RELEASE>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain json yaml" -- "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -W "plain json yaml" -- "${cur}"))
                    return 0
                    ;;
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__release__help)
            opts="list describe install uninstall help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__release__help__describe)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__release__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__release__help__install)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__release__help__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__release__help__uninstall)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__release__install)
            opts="-i -e -c -l -d -s -r -h -V --include --exclude --operator-ns --operator-namespace --cluster --cluster-name --cluster-nodes --cluster-cp-nodes --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version <RELEASE>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --include)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -i)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --exclude)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -e)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --operator-namespace)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --operator-ns)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cluster)
                    COMPREPLY=($(compgen -W "kind minikube" -- "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -W "kind minikube" -- "${cur}"))
                    return 0
                    ;;
                --cluster-name)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cluster-nodes)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cluster-cp-nodes)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__release__list)
            opts="-o -l -d -s -r -h -V --output --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain json yaml" -- "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -W "plain json yaml" -- "${cur}"))
                    return 0
                    ;;
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__release__uninstall)
            opts="-l -d -s -r -h -V --operator-ns --operator-namespace --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version <RELEASE>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --operator-namespace)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --operator-ns)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__stack)
            opts="-l -d -s -r -h -V --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version list describe install help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__stack__describe)
            opts="-o -l -d -s -r -h -V --output --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version <STACK_NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain json yaml" -- "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -W "plain json yaml" -- "${cur}"))
                    return 0
                    ;;
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__stack__help)
            opts="list describe install help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__stack__help__describe)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__stack__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__stack__help__install)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__stack__help__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__stack__install)
            opts="-c -n -l -d -s -r -h -V --skip-release --stack-parameters --parameters --cluster --cluster-name --cluster-nodes --cluster-cp-nodes --operator-ns --operator-namespace --product-ns --product-namespace --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version <STACK_NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --stack-parameters)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --parameters)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cluster)
                    COMPREPLY=($(compgen -W "kind minikube" -- "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -W "kind minikube" -- "${cur}"))
                    return 0
                    ;;
                --cluster-name)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cluster-nodes)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cluster-cp-nodes)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --operator-namespace)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --operator-ns)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --product-namespace)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --product-ns)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -n)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__stack__list)
            opts="-o -l -d -s -r -h -V --output --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain json yaml" -- "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -W "plain json yaml" -- "${cur}"))
                    return 0
                    ;;
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__stacklet)
            opts="-l -d -s -r -h -V --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version credentials list help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__stacklet__credentials)
            opts="-n -l -d -s -r -h -V --operator-ns --operator-namespace --product-ns --product-namespace --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version <PRODUCT_NAME> <STACKLET_NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --operator-namespace)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --operator-ns)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --product-namespace)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --product-ns)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -n)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__stacklet__help)
            opts="credentials list help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__stacklet__help__credentials)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__stacklet__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__stacklet__help__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        stackablectl__stacklet__list)
            opts="-c -o -n -l -d -s -r -h -V --color --show-credentials --output --operator-ns --operator-namespace --product-ns --product-namespace --log-level --no-cache --offline --demo-file --stack-file --release-file --helm-repo-stable --helm-repo-test --helm-repo-dev --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain json yaml" -- "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -W "plain json yaml" -- "${cur}"))
                    return 0
                    ;;
                --operator-namespace)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --operator-ns)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --product-namespace)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --product-ns)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -n)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --demo-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stack-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --release-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-stable)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-test)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --helm-repo-dev)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

complete -F _stackablectl -o bashdefault -o default stackablectl
