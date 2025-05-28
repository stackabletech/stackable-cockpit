use std::{fs::File, io::Write, path::PathBuf, process::Command};

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR is required"));

    let vite_out_dir = out_dir.join("vite-dist");
    // Vite always seems to update the mtime of the project folder on each build, so track the files that we know matter individually
    for tracked_file in [
        "package.json",
        "../yarn.lock",
        "index.html",
        "public",
        "src",
        "tsconfig.json",
        "vite.config.ts",
    ] {
        println!("cargo:rerun-if-changed={tracked_file}");
    }

    let mut vite_command = Command::new("yarn");
    vite_command
        .arg("run")
        .arg("build")
        .arg("--outDir")
        .arg(&vite_out_dir)
        .arg("--base")
        .arg("/ui/");

    let vite_status = vite_command.status();
    match vite_status {
        Ok(vite_status) if vite_status.success() => {}
        _ => panic!("web-ui build failed: command {vite_command:?} resulted in {vite_status:?}"),
    };

    let mut asset_map = phf_codegen::Map::new();
    for asset_file in vite_out_dir.join("assets").read_dir().unwrap() {
        let asset_file = asset_file.unwrap();
        let asset_file_name = asset_file.file_name();
        let asset_file_path = asset_file.path();
        asset_map.entry(
            asset_file_name
                .to_str()
                .expect("asset filename must be valid UTF-8")
                .to_string(),
            &format!("include_bytes!({asset_file_path:?})"),
        );
    }
    write!(
        File::create(out_dir.join("vite-asset-map.rs")).unwrap(),
        "{asset_map}",
        asset_map = asset_map.build()
    )
    .unwrap();
}
