#[cfg(feature = "cc")]
extern crate cbindgen;
#[cfg(feature = "cc")]
use std::env;

fn main() {
    #[cfg(feature = "cc")]
    {
        let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

        cbindgen::Builder::new()
            .with_crate(crate_dir)
            .with_include_guard("INTERFACE_H")
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file("include/interface.h");
    }
}
