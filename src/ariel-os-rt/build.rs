use std::env;
use std::path::PathBuf;

fn main() {
    // Put the linker scripts somewhere the linker can find them
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    std::fs::copy("isr_stack.ld.in", out.join("isr_stack.x")).unwrap();
    std::fs::copy("linkme.x", out.join("linkme.x")).unwrap();
    std::fs::copy("eheap.x", out.join("eheap.x")).unwrap();
    std::fs::copy("keep-stack-sizes.x", out.join("keep-stack-sizes.x")).unwrap();

    println!("cargo:rerun-if-changed=isr_stack.x");
    println!("cargo:rerun-if-changed=linkme.x");
    println!("cargo:rerun-if-changed=eheap.x");
    println!("cargo:rerun-if-changed=keep-stack-sizes.x");

    println!("cargo:rustc-link-search={}", out.display());
}
