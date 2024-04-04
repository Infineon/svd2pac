use std::fs;
use std::path::Path;
use std::process::Command;

#[allow(dead_code)]
pub fn assert_files_eq<T: AsRef<Path>, Q: AsRef<Path>>(ref_file: T, gen_file: Q) {
    let ref_file = fs::read_to_string(ref_file).expect("Unable to read source file");
    let gen_file = fs::read_to_string(gen_file).expect("Unable to read generated file");
    let result = similar::TextDiff::from_lines(&ref_file, &gen_file);
    let diff = format!(
        "{}",
        result
            .unified_diff()
            .context_radius(10)
            .header("ref_file", "new_file")
    );
    assert!(
        !result
            .ops()
            .iter()
            .any(|ops| !matches!(*ops, similar::DiffOp::Equal { .. })),
        "{}",
        diff
    );
}

/// execute cargo build and check that build is successfull
pub fn assert_cargo_build(package_folder: tempfile::TempDir) {
    // Run cargo to build
    let mut command = Command::new("cargo");
    command.arg("build");
    command.current_dir(package_folder.path());

    let exec_result = command.output();

    if exec_result.is_err() {
        // This to preserve the project for further debugging
        let _ = package_folder.into_path();
        panic!("Failed to execute");
    }
    let output_result = exec_result.unwrap();
    if !output_result.status.success() {
        let stdout_msg = std::str::from_utf8(&output_result.stdout)
            .expect("Failed to parse stdout returned from cargo build");
        let stderr_msg = std::str::from_utf8(&output_result.stderr)
            .expect("Failed to parse stderr returned from cargo build");
        eprintln!("Failed compilation of test project stdout: {}", stdout_msg);
        eprintln!("Failed compilation of test project stderr: {}", stderr_msg);
        // This to preserve the project for further debugging
        let _ = package_folder.into_path();
        panic!("Failed compilation of test project");
    }
}

#[allow(dead_code)]
pub fn assert_cargo_test(package_folder: tempfile::TempDir) {
    // Run cargo to build
    let mut command = Command::new("cargo");
    command.arg("test");
    command.current_dir(package_folder.path());

    let exec_result = command.output();

    if exec_result.is_err() {
        // This to preserve the project for further debugging
        let _ = package_folder.into_path();
        panic!("Failed to execute tests");
    }
    if !exec_result.unwrap().status.success() {
        // This to preserve the project for further debugging
        let _ = package_folder.into_path();
        panic!("Failed running tests of test project");
    }
}
