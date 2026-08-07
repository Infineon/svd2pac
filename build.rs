use rustc_version::version;
use std::process::Command;
const AURIX_TEST_TOOLCHAIN: &str = "tricore-htc-none-v11.0.0";
fn detect_aurix_toolchain(toolchain: &str) -> Result<bool, ()> {
    Command::new("rustup")
        .args(["toolchain", "list"])
        .output()
        .map_or(Err(()), |result| {
            let result =
                String::from_utf8(result.stdout).expect("Unable to convert to utf8 string");
            Ok(result.contains(toolchain))
        })
}

fn main() {
    // To avoid warnings related unexpected cfgs when compiling with rustc >=1.80
    // we want to be still compatible with Hightec Rust compiler presently supporting 1.72 version.
    let version = version().unwrap();
    if version.minor >= 80 {
        println!("cargo:rustc-check-cfg=cfg(aurix_tests)");
    }
    // In case of Aurix toolchain enable test of code generated for Aurix microcontroller

    // Check if AURIX_TOOLCHAIN environment variable exists
    if std::env::var("AURIX_TOOLCHAIN").is_ok() {
        println!("cargo:rustc-cfg=aurix_tests");
    } else {
        match detect_aurix_toolchain(AURIX_TEST_TOOLCHAIN) {
            Err(_) => println!(
                "cargo::warning=rustup not available unable to detect presence of Aurix toolchain"
            ),
            Ok(true) => {
                println!("cargo:rustc-cfg=aurix_tests");
                println!("cargo:rustc-env=AURIX_TOOLCHAIN={}", AURIX_TEST_TOOLCHAIN);
            }
            Ok(false) => (),
        }
    }
}
