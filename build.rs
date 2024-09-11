use rustc_version::version;
use std::env;
const RUSTUP_TOOLCHAIN_ID: &str = "RUSTUP_TOOLCHAIN";
fn main() {
    // To avoid warnings related unexpected cfgs when compiling with rustc >=1.80
    // we want to be still compatible with Hightec Rust compiler presently supporting 1.72 version.
    let version = version().unwrap();
    if version.minor >= 80 {
        println!("cargo:rustc-check-cfg=cfg(aurix_tests)");
    }
    // In case of Aurix toolchain enable test of code generated for Aurix microcontroller
    let rustup_toolchain = env::var(RUSTUP_TOOLCHAIN_ID)
        .unwrap_or_else(|_| format!("Unable to to get environment variable {RUSTUP_TOOLCHAIN_ID}"));
    if rustup_toolchain.contains("tricore") {
        println!("cargo:rustc-cfg=aurix_tests");
    }
}
