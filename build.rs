// build.rs
extern crate embed_resource;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        // 使用 embed-resource crate 将 app.manifest 嵌入到 exe 中
        let _ = embed_resource::compile("app.manifest", embed_resource::NONE);
    }
    println!("cargo:rerun-if-changed=app.manifest");
}
