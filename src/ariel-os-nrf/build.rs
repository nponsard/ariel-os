use ld_memory::{Memory, MemorySection};

fn main() {
    if !is_in_current_contexts(&["ariel-os"]) {
        // Platform-independent tooling.
        return;
    }

    let (ram, rom) = if is_in_current_contexts(&["nrf52832"]) {
        (64, 256)
    } else if is_in_current_contexts(&["nrf52833"]) {
        (128, 512)
    } else if is_in_current_contexts(&["nrf52840"]) {
        (256, 1024)
    } else if is_in_current_contexts(&["nrf5340"]) {
        (512, 1024)
    } else if is_in_current_contexts(&["nrf9151", "nrf9160"]) {
        (256, 1024)
    } else {
        panic!("nrf52: please set MCU feature");
    };

    let slot_prefix = if is_in_current_contexts(&["nrf52"]) {
        "NRF52_FLASH"
    } else if is_in_current_contexts(&["nrf5340"]) {
        "NRF5340_FLASH"
    } else if is_in_current_contexts(&["nrf9151"]) {
        "NRF9151_FLASH"
    } else if is_in_current_contexts(&["nrf9160"]) {
        "NRF9160_FLASH"
    } else {
        unreachable!();
    };

    // generate linker script
    let memory = Memory::new()
        .add_section(MemorySection::new("RAM", 0x20000000, ram * 1024))
        .add_section(
            MemorySection::new("FLASH", 0x00000, rom * 1024)
                .pagesize(4096)
                .from_env_with_prefix(slot_prefix),
        );

    memory.to_cargo_outdir("memory.x").expect("wrote memory.x");

    println!("cargo:rerun-if-changed=build.rs");
}

/// Returns whether any of the current `cfg` contexts is one of the given contexts.
fn is_in_current_contexts(contexts: &[&str]) -> bool {
    let Ok(context_var) = std::env::var("CARGO_CFG_CONTEXT") else {
        return false;
    };

    // Contexts cannot include commas.
    context_var.split(',').any(|c| contexts.contains(&c))
}
