
fn main() {
    println!("cargo:rustc-link-search=/opt/vc/lib");
    println!("cargo:rustc-link-lib=mmal_core");
    println!("cargo:rustc-link-lib=mmal_components");
    println!("cargo:rustc-link-lib=mmal_util");
    println!("cargo:rustc-link-lib=mmal_vc_client");
    println!("cargo:rustc-link-lib=vcos");
    println!("cargo:rustc-link-lib=bcm_host");
}

