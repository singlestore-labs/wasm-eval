fn main() {
    let out_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/lib");
    println!("cargo:rustc-link-search=native={}", out_dir);
}