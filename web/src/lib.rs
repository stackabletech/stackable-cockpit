pub const INDEX_HTML: &str = include_str!(concat!(env!("OUT_DIR"), "/vite-dist/index.html"));
pub const ASSETS: phf::Map<&str, &[u8]> = include!(concat!(env!("OUT_DIR"), "/vite-asset-map.rs"));
