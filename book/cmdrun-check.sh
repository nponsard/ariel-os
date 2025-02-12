#!/bin/sh
# This file is run by CI and checks for unintended uses of 'cmdrun'.
# Currently, only the one call to `gen_support_matrix_html.rs` will pass.
#
# This line greps for all occurrences of `<!-- cmdrun ...`, filters out the
# actual command via sed, then compares it to the one command that we
# currently allow. It'll set the exit code based on that comparison.
test "$(grep -r '<!--\s*cmdrun' src | sed 's/.*<!--\s*cmdrun \(.*\) -->.*/\1/')" = '../../doc/gen_support_matrix_html.rs generate ../../doc/support_matrix.yml /dev/stdout'
