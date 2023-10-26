use std::{env, path::PathBuf, process::Command};

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=go-helm-wrapper/main.go");

    let mut cmd = Command::new("go");
    cmd.arg("build")
        .args(["-buildmode", "c-archive"])
        .arg("-o")
        .arg(out_path.join("libgo-helm-wrapper.a"))
        .arg("go-helm-wrapper/main.go")
        .env("CGO_ENABLED", "1");
    cmd.status().expect("Failed to build go-helm-wrapper");

    let bindings = bindgen::builder()
        .header(out_path.join("libgo-helm-wrapper.h").to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Failed to generate Rust bindings from Go header file");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");

    println!("cargo:rustc-link-lib=static=go-helm-wrapper");
    println!(
        "cargo:rustc-link-search=native={}",
        out_path.to_str().unwrap()
    );
}
