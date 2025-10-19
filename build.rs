use std::{env, fs, path::PathBuf};

fn main() {
    // only run when we are building the *web* target
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() != "wasm32" {
        return;
    }

    let out = PathBuf::from("docs"); // same as --out-dir docs
    if !out.exists() {
        return;
    }

    // 1. fix index.html
    let html = fs::read_to_string(out.join("index.html")).unwrap_or_default();
    let html = html
        .replace("=\"/", "=\"") // drop leading slash
        .replace("href=\"assets/", "href=\"public/assets/")
        .replace("src=\"assets/", "src=\"public/assets/");
    fs::write(out.join("index.html"), html).unwrap();

    // 2. fix the JS loader so it looks under /blogy/public/…
    for entry in fs::read_dir(out.join("public").join("assets")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("js") {
            let js = fs::read_to_string(&path).unwrap();
            let js = js.replace("\"/assets/", "\"/blogy/public/assets/");
            fs::write(path, js).unwrap();
        }
    }

    println!("cargo:rerun-if-changed=docs/index.html");
}
