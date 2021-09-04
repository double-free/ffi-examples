use bindgen;
use cc;

use std::env;
use std::path::PathBuf;

// https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() {
    let c_lib_name = "mylib";
    println!("cargo:rerun-if-changed=c-src/{}.c", c_lib_name);
    println!("cargo:rustc-link-lib={}", c_lib_name);

    cc::Build::new()
        .file(format!("c-src/{}.c", c_lib_name))
        .compile(c_lib_name);


    let bindings = bindgen::Builder::default()
        .header(format!("c-src/{}.h", c_lib_name))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join(format!("{}.rs", c_lib_name)))
        .expect(format!("Can not write bindings to {:?}", out_path).as_str());
}
