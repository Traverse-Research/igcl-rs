use std::path::Path;

fn main() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));

    let header_path = crate_root.join("wrapper.h");

    let msrv = bindgen::RustTarget::stable(74, 0).unwrap();

    let bindings = bindgen::Builder::default()
        .header(header_path.to_string_lossy())
        .rust_target(msrv)
        .derive_debug(true)
        .layout_tests(false)
        .impl_debug(true)
        .derive_default(true)
        .clang_arg(format!(
            "-I{}",
            crate_root
                .join("../vendor/drivers.gpu.control-library/include/")
                .display()
        ))
        .dynamic_link_require_all(true)
        .dynamic_library_name("ControlLib")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        .clang_arg("-Wno-pragma-once-outside-header")
        .allowlist_item("(ctl|CTL)\\w+")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .generate()
        .expect("failed to generate igcl bindings");

    bindings
        .write_to_file(crate_root.join("../src/ffi.rs"))
        .expect("Couldn't write bindings!");
}
