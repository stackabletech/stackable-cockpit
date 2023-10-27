use std::{
    env::{self, VarError},
    path::PathBuf,
    process::Command,
};

use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Failed to find env var"))]
    EnvVarNotFound { source: VarError },

    #[snafu(display("Unsupported GOARCH: {arch}"))]
    UnsupportedGoArch { arch: String },

    #[snafu(display("Unsupported GOOS: {os}"))]
    UnsupportedGoOs { os: String },
}

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=go-helm-wrapper/main.go");

    let cc = cc::Build::new().try_get_compiler().unwrap();
    let goarch = get_goarch().unwrap();
    let goos = get_goos().unwrap();

    let mut cmd = Command::new("go");
    cmd.arg("build")
        .args(["-buildmode", "c-archive"])
        .arg("-o")
        .arg(out_path.join("libgo-helm-wrapper.a"))
        .arg("go-helm-wrapper/main.go")
        .env("CGO_ENABLED", "1")
        .env("GOARCH", goarch)
        .env("GOOS", goos)
        .env("CC", format!("'{}'", cc.path().display()));

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

fn get_goarch() -> Result<String, Error> {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").context(EnvVarNotFoundSnafu)?;

    let arch = match arch.as_str() {
        "x86" => "386",
        "x86_64" => "amd64",
        "mips" => "mips",
        "powerpc" => "ppc",
        "powerpc64" => "ppc64",
        "arm" => "arm",
        "aarch64" => "arm64",
        _ => return UnsupportedGoArchSnafu { arch }.fail(),
    };

    Ok(arch.into())
}

fn get_goos() -> Result<String, Error> {
    let os = env::var("CARGO_CFG_TARGET_OS").context(EnvVarNotFoundSnafu)?;

    let os = match os.as_str() {
        "windows" => "windows",
        "macos" => "darwin",
        "ios" => "darwin",
        "linux" => "linux",
        "android" => "android",
        "freebsd" => "freebsd",
        "dragonfly" => "dragonfly",
        "openbsd" => "openbsd",
        "netbsd" => "netbsd",
        _ => return UnsupportedGoOsSnafu { os }.fail(),
    };

    Ok(os.into())
}
