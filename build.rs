use std::env;
const RUSTUP_TOOLCHAIN_ID: &str = "RUSTUP_TOOLCHAIN";
fn main() {
    // In case of Aurix toolchain enable test of code generated for Aurix microcontroller
    let rustup_toolchain = env::var(RUSTUP_TOOLCHAIN_ID)
        .unwrap_or_else(|_| format!("Unable to to get environment variable {RUSTUP_TOOLCHAIN_ID}"));
    if rustup_toolchain.contains("tricore") {
        println!("cargo:rustc-cfg=aurix_tests");
    }
}
