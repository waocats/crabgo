fn main() {
    println!(
        "crabgo:rustc-env=NATIVE_ARCH={}",
        std::env::var("TARGET").unwrap()
    );
    println!("crabgo:rerun-if-changed=build.rs");
}
