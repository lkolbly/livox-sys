use std::env;
use std::path::PathBuf;
use cmake;

fn main() {
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dylib=boost_system");
    println!("cargo:rustc-link-lib=dylib=apr-1");

    // Run bindgen
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Build the Livox SDK
    let dst = cmake::build("src/Livox-SDK");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=livox_sdk_static");
}
