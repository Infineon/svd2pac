use std::fs;
use std::path::Path;
use std::process::{Command, exit};

#[allow(dead_code)]
pub fn assert_files_eq<T: AsRef<Path>, Q: AsRef<Path>>(ref_file: T, gen_file: Q) {
    let ref_file_path = ref_file.as_ref();
    let gen_file_path = gen_file.as_ref();
    let ref_content = fs::read_to_string(ref_file_path).expect("Unable to read source file");
    let gen_content = fs::read_to_string(gen_file_path).expect("Unable to read generated file");

    const SKIP_MARKER: &str = "Generated from SVD 1.2, with svd2pac";

    let mut ref_lines = ref_content.lines().enumerate();
    let mut gen_lines = gen_content.lines().enumerate();

    loop {
        let ref_next = ref_lines.next();
        let gen_next = gen_lines.next();

        match (ref_next, gen_next) {
            (Some((ref_idx, ref_line)), Some((gen_idx, gen_line))) => {
                if ref_line.contains(SKIP_MARKER) && gen_line.contains(SKIP_MARKER) {
                    continue;
                }

                let ref_line_no = ref_idx + 1;
                let gen_line_no = gen_idx + 1;
                if ref_line != gen_line {
                    eprintln!(
                        "Mismatch in {} at line {} (compared with {} line {}).\nreference: {:?}\ngenerated: {:?}",
                        ref_file_path.display(),
                        ref_line_no,
                        gen_file_path.display(),
                        gen_line_no,
                        ref_line,
                        gen_line
                    );
                    exit(-1);
                }
            }
            (None, None) => break,
            (Some((ref_idx, _)), None) => {
                eprintln!(
                    "Line count mismatch: {} has extra content starting at line {}, but {} ended",
                    ref_file_path.display(),
                    ref_idx + 1,
                    gen_file_path.display()
                );
                exit(-1);
            }
            (None, Some((gen_idx, _))) => {
                eprintln!(
                    "Line count mismatch: {} has extra content starting at line {}, but {} ended",
                    gen_file_path.display(),
                    gen_idx + 1,
                    ref_file_path.display()
                );
                exit(-1);
            }
        }
    }
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

/// Compare two folders recursively and assert they are identical
/// - Verifies that all files are present with the same name in both folders
/// - Verifies there are no extra files
/// - Uses assert_files_eq to compare the content of files with the same relative path
#[allow(dead_code)]
pub fn assert_folders_eq<T: AsRef<Path>, Q: AsRef<Path>>(ref_folder: T, gen_folder: Q) {
    let ref_folder = ref_folder.as_ref();
    let gen_folder = gen_folder.as_ref();

    // Helper function to collect all files in a folder recursively
    fn collect_files_recursive(
        folder: &Path,
        base_folder: &Path,
        files: &mut Vec<std::path::PathBuf>,
    ) {
        if let Ok(entries) = fs::read_dir(folder) {
            for entry in entries.flatten() {
                let path = entry.path();
                // Skip directories and files that are not relevant for comparison (like Cargo.lock)
                if path.is_file() && path.file_name() != Some(std::ffi::OsStr::new("Cargo.lock")) {
                    if let Ok(relative) = path.strip_prefix(base_folder) {
                        files.push(relative.to_path_buf());
                    }
                } else if path.is_dir() {
                    collect_files_recursive(&path, base_folder, files);
                }
            }
        }
    }

    // Collect all files from both folders
    let mut ref_files = Vec::new();
    collect_files_recursive(ref_folder, ref_folder, &mut ref_files);
    ref_files.sort();

    let mut gen_files = Vec::new();
    collect_files_recursive(gen_folder, gen_folder, &mut gen_files);
    gen_files.sort();

    // Assert that file lists match and print only the first difference
    if let Some((idx, (ref_file, gen_file))) = ref_files
        .iter()
        .zip(gen_files.iter())
        .enumerate()
        .find(|(_, (ref_file, gen_file))| ref_file != gen_file)
    {
        eprintln!(
            "File lists differ at index {}. reference: {:?}, generated: {:?}",
            idx, ref_file, gen_file
        );
        exit(-1);
    }

    if ref_files.len() != gen_files.len() {
        eprintln!(
            "File list length mismatch. first extra file in {}: {:?}; first extra file in {}: {:?}",
            ref_folder.display(),
            ref_files.get(gen_files.len()),
            gen_folder.display(),
            gen_files.get(ref_files.len())
        );
        exit(-1);
    }

    // Compare content of each file
    for file in ref_files {
        let ref_path = ref_folder.join(&file);
        let gen_path = gen_folder.join(&file);
        assert_files_eq(&ref_path, &gen_path);
    }
}
