use std::env;
use std::path::PathBuf;

fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rustc-link-search={}", out.display());
}
