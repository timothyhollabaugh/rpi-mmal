extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=/opt/vc/lib");
    println!("cargo:rustc-link-lib=mmal_core");
    println!("cargo:rustc-link-lib=mmal_components");
    println!("cargo:rustc-link-lib=mmal_util");
    println!("cargo:rustc-link-lib=mmal_vc_client");
    println!("cargo:rustc-link-lib=vcos");
    println!("cargo:rustc-link-lib=bcm_host");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/opt/vc/include")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("{:?}", env::var("OUT_DIR"));

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings");
}

