use std::io::{BufRead, Write};
use std::path::Path;
use std::process::{Command, exit};
use std::{fs, io};

const SKIP_MARKER: &str = "Generated from SVD 1.2, with svd2pac";

#[allow(dead_code)]
pub fn assert_files_eq<T: AsRef<Path>, Q: AsRef<Path>>(
    ref_file: T,
    gen_file: Q,
) -> Result<(), String> {
    let ref_file_path = ref_file.as_ref();
    let gen_file_path = gen_file.as_ref();
    let ref_content = fs::read_to_string(ref_file_path).expect("Unable to read source file");
    let gen_content = fs::read_to_string(gen_file_path).expect("Unable to read generated file");

    let mut ref_lines = ref_content.lines().enumerate();
    let mut gen_lines = gen_content.lines().enumerate();

    loop {
        let ref_next = ref_lines.next();
        let mut gen_next = gen_lines.next();

        // Skip lines in the generated file that contain the skip marker.
        // These lines are excluded from the reference files.
        if let Some((_, line)) = gen_next
            && line.contains(SKIP_MARKER)
        {
            gen_next = gen_lines.next();
        }

        match (ref_next, gen_next) {
            (Some((ref_idx, ref_line)), Some((gen_idx, gen_line))) => {
                if ref_line != gen_line {
                    return Err(format!(
                        "Mismatch in {} at line {} (compared with {} line {}).\nreference: {:?}\ngenerated: {:?}",
                        ref_file_path.display(),
                        ref_idx + 1,
                        gen_file_path.display(),
                        gen_idx + 1,
                        ref_line,
                        gen_line
                    ));
                }
            }
            (None, None) => break,
            (Some((ref_idx, _)), None) => {
                return Err(format!(
                    "Line count mismatch: {} has extra content starting at line {}, but {} ended",
                    ref_file_path.display(),
                    ref_idx + 1,
                    gen_file_path.display()
                ));
            }
            (None, Some((gen_idx, _))) => {
                return Err(format!(
                    "Line count mismatch: {} has extra content starting at line {}, but {} ended",
                    gen_file_path.display(),
                    gen_idx + 1,
                    ref_file_path.display()
                ));
            }
        }
    }

    Ok(())
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
pub fn check_reference_output<T: AsRef<Path>, Q: AsRef<Path>>(ref_folder: T, gen_folder: Q) {
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

    let bless = std::env::var("SVD2PAC_BLESS_TESTS").as_deref() == Ok("1");

    if bless {
        // Empty the reference folder.
        if ref_folder.exists() {
            fs::remove_dir_all(ref_folder).expect("Failed to remove reference folder");
        }

        // Copy the generated files to the reference folder, overwriting existing files
        for file in &gen_files {
            let gen_path = gen_folder.join(file);
            let ref_path = ref_folder.join(file);
            if let Some(parent) = ref_path.parent() {
                fs::create_dir_all(parent).expect("Failed to create parent directories");
            }

            // Copy the file contents, ignoring the skip marker line if present
            let gen_file = io::BufReader::new(
                fs::File::open(&gen_path).expect("Failed to open generated file"),
            );
            let mut ref_file = io::BufWriter::new(
                fs::File::create(&ref_path).expect("Failed to create reference file"),
            );

            for line in gen_file.lines() {
                let line = line.expect("Failed to read from generated file");
                if !line.contains(SKIP_MARKER) {
                    writeln!(&mut ref_file, "{}", line).expect("Failed to write to reference file");
                }
            }
        }
    } else {
        fn handle_error(err: String) -> ! {
            panic!(
                "{}\nnote: to update the reference output, run with SVD2PAC_BLESS_TESTS=1",
                err
            );
        }

        // Assert that file lists match and print only the first difference
        if let Some((idx, (ref_file, gen_file))) = ref_files
            .iter()
            .zip(gen_files.iter())
            .enumerate()
            .find(|(_, (ref_file, gen_file))| ref_file != gen_file)
        {
            handle_error(format!(
                "File lists differ at index {}. reference: {:?}, generated: {:?}",
                idx, ref_file, gen_file
            ));
        }

        if ref_files.len() != gen_files.len() {
            handle_error(format!(
                "File list length mismatch. first extra file in {}: {:?}; first extra file in {}: {:?}",
                ref_folder.display(),
                ref_files.get(gen_files.len()),
                gen_folder.display(),
                gen_files.get(ref_files.len())
            ));
        }

        // Compare content of each file
        for file in ref_files {
            let ref_path = ref_folder.join(&file);
            let gen_path = gen_folder.join(&file);
            assert_files_eq(&ref_path, &gen_path).unwrap_or_else(|e| handle_error(e));
        }
    }
}
