use std::env;

use gobuild::BuildMode;

const ENV_GO_HELM_WRAPPER: &str = "GO_HELM_WRAPPER";

fn main() {
    // cgo requires an explicit dependency on libresolv on some platforms (such as Red Hat Enterprise Linux 8 and derivatives)
    println!("cargo:rustc-link-lib=resolv");
    println!("cargo:rerun-if-env-changed={ENV_GO_HELM_WRAPPER}");
    match env::var(ENV_GO_HELM_WRAPPER) {
        Ok(go_helm_wrapper) => {
            // Reuse pre-built helm wrapper if possible
            eprintln!("Reusing pre-built go-helm-wrapper ({go_helm_wrapper:?})");
            println!("cargo:rustc-link-lib=static:+verbatim={go_helm_wrapper}");
        }
        Err(env::VarError::NotPresent) => {
            gobuild::Build::new()
                .file("go-helm-wrapper/main.go")
                .buildmode(BuildMode::CArchive)
                .compile("go-helm-wrapper");
        }
        Err(err @ env::VarError::NotUnicode(..)) => {
            panic!("{ENV_GO_HELM_WRAPPER} must be valid unicode: {err}");
        }
    }
}
