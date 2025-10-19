use std::{env, fs, path::PathBuf};

fn main() {
    // only touch output when we are building for wasm32
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() != "wasm32" {
        return;
    }

    let out = PathBuf::from("docs");
    if !out.exists() {
        return;
    }

    /* ---------- 1.  patch index.html  ---------- */
    let html_path = out.join("index.html");
    let html = fs::read_to_string(&html_path).unwrap_or_default();
    let html = html
        .replace("=\"/", "=\"") // drop leading slash
        .replace("href=\"assets/", "href=\"assets/") // now flat
        .replace("src=\"assets/", "src=\"assets/");
    fs::write(html_path, html).unwrap();

    /* ---------- 2.  move docs/public/\* → docs/  ---------- */
    let public = out.join("public");
    if public.is_dir() {
        for entry in fs::read_dir(&public).unwrap() {
            let e = entry.unwrap();
            let name = e.file_name();
            let src = e.path();
            let dst = out.join(&name);
            fs::rename(src, dst).unwrap();
        }
        fs::remove_dir(public).unwrap();
    }

    /* ---------- 3.  patch the JS loader (now directly in docs/)  ---------- */
    for entry in fs::read_dir(out.join("assets")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("js") {
            let js = fs::read_to_string(&path).unwrap();
            let js = js.replace("\"/assets/", "\"/blogy/assets/");
            fs::write(path, js).unwrap();
        }
    }

    println!("cargo:rerun-if-changed=docs/index.html");
}
