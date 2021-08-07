#!/bin/bash
# shellcheck disable=SC2207

function __hs {
    local cur
    cur="${COMP_WORDS[COMP_CWORD]}"

    if [ "$COMP_CWORD" -ge 1 ]; then
        COMPREPLY=($(compgen -W '--help --file' -- "$cur"))
        local IFS=$'\n' # Handle filenames with spaces.
        COMPREPLY+=($(compgen -f -- "$cur"))
    fi
}

complete -o bashdefault -o default -o filenames -F __hs hs
