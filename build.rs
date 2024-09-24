use regex::Regex;
use rustc_version::version;
use std::process::Command;

fn detect_aurix_toolchain() -> Result<Option<String>, ()> {
    Command::new("rustup")
        .args(["toolchain", "list"])
        .output()
        .map_or(Err(()), |result| {
            let re = Regex::new(r"tricore-htc-none.+").unwrap();
            let result =
                String::from_utf8(result.stdout).expect("Unable to convert to utf8 string");
            Ok(re.find(&result).map(|m| m.as_str().to_string()))
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
    match detect_aurix_toolchain() {
        Err(_) => println!(
            "cargo::warning=rustup not available unable to detect presence of Aurix toolchain"
        ),
        Ok(Some(aurix_toolchain)) => {
            println!("cargo:rustc-cfg=aurix_tests");
            println!("cargo:rustc-env=AURIX_TOOLCHAIN={}", aurix_toolchain);
        }
        Ok(None) => (),
    }
}
