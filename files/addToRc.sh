#@IgnoreInspection BashAddShebang
function ttmp {
    directory=`~/.cargo/bin/ttmp`
    if [ -d ${directory} ] && [ -x ${directory} ]; then
        cd `~/.cargo/bin/ttmp`
    else
        >&2 echo "Directory ${directory} does not exists or is not accessible"
    fi
}
