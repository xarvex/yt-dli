# shellcheck shell=sh

vhs_sandbox=$(mktemp -d)
trap 'rm -rf -- "${vhs_sandbox}"' EXIT

export HOME="${vhs_sandbox}"
export XDG_CONFIG_HOME="${vhs_sandbox}/.config"

ln -s "$(readlink -f media/vhs/config)" "${XDG_CONFIG_HOME}"

clear
