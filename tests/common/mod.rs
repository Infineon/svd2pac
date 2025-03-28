use std::fs;
use std::path::Path;
use std::process::{exit, Command};

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

#[allow(dead_code)]
pub enum CargoCommand {
    Build,
    Clippy,
    Clean,
}

impl std::fmt::Display for CargoCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let command_str = match self {
            CargoCommand::Build => "build",
            CargoCommand::Clippy => "clippy",
            CargoCommand::Clean => "clean",
        };
        write!(f, "{}", command_str)
    }
}

/// execute cargo command cargo  and check that build is successful
pub fn assert_cargo_command(
    package_folder: &tempfile::TempDir,
    cargo_command: CargoCommand,
    toolchain_override: Option<String>,
) {
    // Run cargo to build
    let mut command = Command::new("cargo");
    let toolchain_id = if let Some(ref toolchain_id) = toolchain_override {
        command.arg(format!("+{}", toolchain_id));
        toolchain_id
    } else {
        "default"
    };
    match cargo_command {
        CargoCommand::Build => command.arg("build"),
        CargoCommand::Clippy => command.arg("clippy"),
        CargoCommand::Clean => command.arg("clean"),
    };
    command.current_dir(package_folder.path());
    let exec_result = command.output();

    if exec_result.is_err() {
        eprintln!("Failed to execute using toolchain: {}", toolchain_id);
        // This to preserve the temp folders for further debugging
        exit(-1);
    }
    let output_result = exec_result.unwrap();

    let stdout_msg = std::str::from_utf8(&output_result.stdout)
        .expect("Failed to parse stdout returned from cargo build");
    check_warning_message(stdout_msg);
    let stderr_msg = std::str::from_utf8(&output_result.stderr)
        .expect("Failed to parse stderr returned from cargo build");
    check_warning_message(stderr_msg);

    if !output_result.status.success() {
        eprintln!("Error stdout: {}", stdout_msg);
        eprintln!("Error stderr: {}", stderr_msg);
        eprintln!(
            "Failed cargo {} of test project using toolchain: {}",
            cargo_command, toolchain_id
        );
        // This to preserve the temp folders for further debugging
        exit(-1);
    }
}

fn check_warning_message(msg: &str) {
    // Check for warnings in stdout
    if msg.to_lowercase().contains("warning") {
        eprintln!("Warning detected: {}", msg);
        // This to preserve the temp folders for further debugging
        exit(-1);
    }
}

#[allow(dead_code)]
pub fn assert_cargo_test(package_folder: &tempfile::TempDir, toolchain_override: Option<String>) {
    Command::new("cargo")
        .arg("clean")
        .current_dir(package_folder.path())
        .output()
        .expect("Failed to clean package");
    // Run cargo to build
    let mut command = Command::new("cargo");
    let toolchain_id = if let Some(ref toolchain_id) = toolchain_override {
        command.arg(format!("+{}", toolchain_id));
        toolchain_id
    } else {
        "default"
    };
    command.arg("test");
    command.current_dir(package_folder.path());

    let exec_result = command.output();

    if exec_result.is_err() {
        eprintln!("Failed to execute tests using toolchain: {}", toolchain_id);
        // This to preserve the temp folders for further debugging
        exit(-1);
    }
    if !exec_result.unwrap().status.success() {
        eprintln!(
            "Failed running tests of test project using toolchain: {}",
            toolchain_id
        );
        // This to preserve the temp folders for further debugging
        exit(-1);
    }
}
