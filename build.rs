use std::env;

fn main() {
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rerun-if-changed=wasm-libs/libed25519.a");

    if env::var("CARGO_CFG_TARGET_ARCH").as_deref() == Ok("wasm32") {
        println!("cargo:rustc-link-lib=static=ed25519");
        println!("cargo:rustc-link-search=native={}/wasm-libs", src_dir);
    }
}
