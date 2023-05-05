use std::{fs::File, io::Write, path::PathBuf, process::Command};

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR is required"));

    #[cfg(feature = "ui")]
    {
        let webui_src_dir = "../../web";
        let webui_out_dir = out_dir.join("web");
        // Vite always seems to update the mtime of the project folder on each build, so track the files that we know matter individually
        for tracked_file in [
            "package.json",
            "node_modules",
            "index.html",
            "public",
            "src",
            "tsconfig.json",
            "vite.config.ts",
        ] {
            println!("cargo:rerun-if-changed={webui_src_dir}/{tracked_file}");
        }
        let vite_status = Command::new("pnpm")
            .arg("run")
            .arg("build")
            .arg("--outDir")
            .arg(&webui_out_dir)
            .arg("--base")
            .arg("/ui/")
            .current_dir(webui_src_dir)
            .status()
            .unwrap();
        if !vite_status.success() {
            panic!("web-ui build failed: {vite_status}");
        }
        let mut asset_map = phf_codegen::Map::new();
        for asset_file in webui_out_dir.join("assets").read_dir().unwrap() {
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
            File::create(out_dir.join("web-asset-map.rs")).unwrap(),
            "{}",
            asset_map.build()
        )
        .unwrap();
    }
}
