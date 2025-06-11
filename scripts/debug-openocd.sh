#!/bin/bash

set -x
openocd $OPENOCD_ARGS -c 'init' -c 'targets' -c "adapter speed 5000" -c "gdb_port 1337"
