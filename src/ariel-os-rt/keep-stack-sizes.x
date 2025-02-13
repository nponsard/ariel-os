/* SPDX-FileCopyrightText: The Rust Project Developers (see https://thanks.rust-lang.org)
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 *
 * As recommended on https://doc.rust-lang.org/beta/unstable-book/compiler-flags/emit-stack-sizes.html */
SECTIONS
{
  /* `INFO` makes the section not allocatable so it won't be loaded into memory */
  .stack_sizes (INFO) :
  {
    KEEP(*(.stack_sizes));
  }
}
