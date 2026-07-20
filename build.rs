use std::fs;
use std::path::Path;

fn main() {
    let libharu_dir = Path::new("vendor/libharu");
    let src_dir = libharu_dir.join("src");
    let include_dir = libharu_dir.join("include");

    let mut builder = cc::Build::new();
    builder
        .include(&include_dir)
        .define("LIBHPDF_HAVE_ZLIB", Some("1"))
        .warnings(false);

    // List of files to exclude from static compilation
    let excluded = [
        "hpdf_doc_png.c",
        "hpdf_image_png.c",
        "hpdf_3dmeasure.c",
        "hpdf_u3d.c",
        "hpdf_exdata.c",
    ];

    if let Ok(entries) = fs::read_dir(&src_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("c") {
                let filename = path.file_name().unwrap().to_str().unwrap();
                if !excluded.contains(&filename) {
                    builder.file(path);
                }
            }
        }
    }

    builder.compile("haru");

    println!("cargo:rustc-link-lib=z");
    println!("cargo:rerun-if-changed=vendor/libharu");
    println!("cargo:rerun-if-changed=build.rs");
}
