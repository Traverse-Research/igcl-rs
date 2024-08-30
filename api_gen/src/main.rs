use std::path::Path;

fn main() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));

    let header_path = crate_root.join("wrapper.h");

    let bindings = bindgen::Builder::default()
        .header(header_path.to_string_lossy())
        .clang_arg(format!(
            "-I{}",
            crate_root
                .join("../vendor/drivers.gpu.control-library/include/")
                .display()
        ))
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        .clang_arg("-Wno-pragma-once-outside-header")
        .allowlist_item("(ctl|CTL)\\w+")
        .generate()
        .expect("failed to generate igcl bindings");

    bindings
        .write_to_file(crate_root.join("../src/ffi.rs"))
        .expect("Couldn't write bindings!");
}
