#!/usr/bin/env bash
# Record the hello world example. Used together with termtosvg and some manual work
# This starts an empty dash subshell and executes the commands by typing them at
# steady pace via xdotool. dash is used as subshell because it doesn't print
# 'exit' on exit.
# usage:
# termtosvg -D 3000 doc/hello-world_render.svg -t doc/recorder_template.svg -c ./recorder.sh -g 100x23
#
# When the typing has stopped, switch to a next console and end this script via `kill`
#
# Be sure to have the repo somewhere anonymous, such as in /tmp to not leak personal info

PSDUMMY="\033[01;34m\$\033[00m"
echo -en "$PSDUMMY "
xdotool type --delay 75 "laze -C examples/hello-world build -b nrf52840dk run"
xdotool sleep 2
xdotool key return

export PS1=""
dash
