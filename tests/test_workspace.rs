mod common;
use common::*;
use fs_extra::dir::CopyOptions;
use std::fs;
use std::{env, path::Path};
use svd2pac::main_parse_arguments;
use toml_edit::{Array, Document, Table, array, value};

/// Patch the generated root `Cargo.toml` so it can build the bundled `main.rs`:
/// enable the requested default features and add a `[[bin]]` target.
fn patch_root_cargo_toml(root: &Path, default_features: &[&str]) {
    let cargo_path = root.join("Cargo.toml");
    let old_toml = fs::read_to_string(&cargo_path).expect("Unable to read toml file");
    let mut parsed_toml = old_toml
        .parse::<Document>()
        .expect("Unable to parse toml file");
    let mut default_table = Array::new();
    for feature in default_features {
        default_table.push(*feature);
    }
    parsed_toml["features"]["default"] = value(default_table);
    parsed_toml["bin"] = array();
    let bin_array = parsed_toml["bin"].as_array_of_tables_mut().unwrap();

    let mut bin_table = Table::new();
    bin_table["name"] = value("main");
    bin_table["test"] = value(false);
    bin_table["bench"] = value(false);
    bin_array.push(bin_table);
    fs::write(&cargo_path, parsed_toml.to_string()).expect("Unable to write toml file");
}

/// Test generic target workspace code generation.
#[test]
fn compile_workspace_generic() {
    let xml_path = concat!(env!("CARGO_MANIFEST_DIR"), "/test_svd/simple.xml");

    // Temp folder that should be deleted in case of test success.
    let generated_code_folder = tempfile::tempdir_in(env::current_dir().unwrap()).unwrap();
    let args = [
        "",
        xml_path,
        generated_code_folder.path().to_str().unwrap(),
        "--workspace-generation",
    ];
    main_parse_arguments(args);

    // Patch toml and add required files.
    patch_root_cargo_toml(generated_code_folder.path(), &["all"]);

    fs_extra::dir::copy(
        "./tests/resources/project_files_generic",
        generated_code_folder.path(),
        &CopyOptions::new().content_only(true).overwrite(true),
    )
    .expect("Failed to copy required files to build cargo project");

    let license_path = generated_code_folder.path().join("LICENSE.txt");
    assert!(license_path.exists(), "Not found LICENSE.txt");

    assert_cargo_command(&generated_code_folder, CargoCommand::Build, None);
    assert_cargo_command(&generated_code_folder, CargoCommand::Clippy, None);
    assert_cargo_command(&generated_code_folder, CargoCommand::Clean, None);
}

/// Test tracing workspace code generation.
#[test]
fn compile_workspace_tracing() {
    let xml_path = concat!(env!("CARGO_MANIFEST_DIR"), "/test_svd/simple.xml");

    // Temp folder that should be deleted in case of test success.
    let generated_code_folder = tempfile::tempdir_in(env::current_dir().unwrap()).unwrap();
    let generated_test_folder = tempfile::tempdir_in(env::current_dir().unwrap()).unwrap();
    let args = [
        "",
        xml_path,
        generated_code_folder.path().to_str().unwrap(),
        "--tracing",
        "--workspace-generation",
    ];
    main_parse_arguments(args);

    // Patch toml and add required files.
    patch_root_cargo_toml(generated_code_folder.path(), &["all", "tracing"]);

    fs_extra::dir::copy(
        "./tests/resources/project_files_tracing",
        generated_code_folder.path(),
        &CopyOptions::new().content_only(true).overwrite(true),
    )
    .expect("Failed to copy required files to build cargo project");
    fs_extra::dir::copy(
        generated_code_folder.path(),
        generated_test_folder.path(),
        &CopyOptions::new().content_only(true).overwrite(true),
    )
    .expect("Failed to copy generated files to test cargo project");

    assert_cargo_command(&generated_code_folder, CargoCommand::Build, None);
    assert_cargo_command(&generated_code_folder, CargoCommand::Clippy, None);
    assert_cargo_test(&generated_test_folder, None);
    assert_cargo_command(&generated_code_folder, CargoCommand::Clean, None);
}

/// Test cortex-m workspace code generation.
#[test]
fn compile_workspace_cortex_m() {
    let xml_path = concat!(env!("CARGO_MANIFEST_DIR"), "/test_svd/simple.xml");

    // Temp folder that becomes the top-level workspace root.
    let workspace_folder = tempfile::tempdir_in(env::current_dir().unwrap()).unwrap();
    let args = [
        "",
        "--target=cortex-m",
        xml_path,
        workspace_folder.path().to_str().unwrap(),
        "--workspace-generation",
    ];
    main_parse_arguments(args);

    // Copy only the `cortex_test` crate (not the outer workspace Cargo.toml,
    // which would otherwise conflict with the generated workspace root).
    fs_extra::dir::copy(
        "./tests/resources/project_files_cortex_m/cortex_test",
        workspace_folder.path(),
        &CopyOptions::new().overwrite(true),
    )
    .expect("Failed to copy required files to build cargo project");

    // Patch the generated root Cargo.toml to add `cortex_test` as a member.
    let cargo_path = workspace_folder.path().join("Cargo.toml");
    let old_toml = fs::read_to_string(&cargo_path).expect("Unable to read toml file");
    let mut parsed_toml = old_toml
        .parse::<Document>()
        .expect("Unable to parse toml file");
    parsed_toml["workspace"]["members"]
        .as_array_mut()
        .expect("workspace.members is not an array")
        .push("cortex_test");
    fs::write(&cargo_path, parsed_toml.to_string()).expect("Unable to write toml file");

    // Patch the `cortex_test` Cargo.toml: the PAC crate now lives at the
    // workspace root, so its dependency path becomes `..`.
    let cortex_cargo_path = workspace_folder.path().join("cortex_test/Cargo.toml");
    let old_cortex_toml =
        fs::read_to_string(&cortex_cargo_path).expect("Unable to read cortex_test toml file");
    let mut parsed_cortex_toml = old_cortex_toml
        .parse::<Document>()
        .expect("Unable to parse cortex_test toml file");
    parsed_cortex_toml["dependencies"]["test_pac"]["path"] = value("..");
    fs::write(&cortex_cargo_path, parsed_cortex_toml.to_string())
        .expect("Unable to write cortex_test toml file");

    let license_path = workspace_folder.path().join("LICENSE.txt");
    assert!(license_path.exists(), "Not found LICENSE.txt");

    assert_cargo_command(&workspace_folder, CargoCommand::Build, None);
    assert_cargo_command(&workspace_folder, CargoCommand::Clippy, None);
    assert_cargo_command(&workspace_folder, CargoCommand::Clean, None);
}

/// Test aurix (with CSFR vendor extensions) workspace code generation.
/// Requires the Aurix Rust toolchain (AURIX_TOOLCHAIN env var); only built
/// when the `aurix_tests` cfg is enabled.
#[cfg(aurix_tests)]
#[test]
fn compile_workspace_aurix() {
    let xml_path = concat!(env!("CARGO_MANIFEST_DIR"), "/test_svd/simple.xml");

    // Temp folder that becomes the top-level workspace root.
    let workspace_folder = tempfile::tempdir_in(env::current_dir().unwrap()).unwrap();
    let args = [
        "",
        "--target=aurix",
        xml_path,
        workspace_folder.path().to_str().unwrap(),
        "--workspace-generation",
    ];
    main_parse_arguments(args);

    // Enable the aggregate `all` feature plus exactly one CSFR cpu, and add the
    // `[[bin]]` target for the bundled main.rs. `csfr_cpuN` is intentionally not
    // part of `all`, so it must be selected explicitly.
    patch_root_cargo_toml(workspace_folder.path(), &["all", "csfr_cpu0"]);

    // Add the `tc162-rt` path dependency (used by the bundled main.rs) and
    // register it as a workspace member. The copied resources contain no
    // top-level Cargo.toml and `tc162-rt/Cargo.toml` declares no `[workspace]`,
    // so member-patching does not conflict with the generated workspace root.
    let cargo_path = workspace_folder.path().join("Cargo.toml");
    let old_toml = fs::read_to_string(&cargo_path).expect("Unable to read toml file");
    let mut parsed_toml = old_toml
        .parse::<Document>()
        .expect("Unable to parse toml file");
    parsed_toml["dependencies"]["tc162-rt"]["path"] = value("tc162-rt");
    parsed_toml["workspace"]["members"]
        .as_array_mut()
        .expect("workspace.members is not an array")
        .push("tc162-rt");
    fs::write(&cargo_path, parsed_toml.to_string()).expect("Unable to write toml file");

    // Copy the aurix project files: main.rs at src/bin, tc162-rt runtime,
    // linker-scripts and the top-level .cargo/config.toml (sets the build
    // target for the whole workspace).
    fs_extra::dir::copy(
        "./tests/resources/project_files_aurix",
        workspace_folder.path(),
        &CopyOptions::new().content_only(true).overwrite(true),
    )
    .expect("Failed to copy required files to build cargo project");

    let license_path = workspace_folder.path().join("LICENSE.txt");
    assert!(license_path.exists(), "Not found LICENSE.txt");

    assert_cargo_command(
        &workspace_folder,
        CargoCommand::Build,
        Some(env!("AURIX_TOOLCHAIN").to_string()),
    );
    assert_cargo_command(
        &workspace_folder,
        CargoCommand::Clean,
        Some(env!("AURIX_TOOLCHAIN").to_string()),
    );
}

/// Test aurix (with CSFR vendor extensions) workspace code generation with the
/// tracing code generated but the `tracing` feature left disabled.
/// Requires the Aurix Rust toolchain (AURIX_TOOLCHAIN env var); only built
/// when the `aurix_tests` cfg is enabled.
#[cfg(aurix_tests)]
#[test]
fn compile_workspace_aurix_tracing() {
    let xml_path = concat!(env!("CARGO_MANIFEST_DIR"), "/test_svd/simple.xml");

    // Temp folder that becomes the top-level workspace root.
    let workspace_folder = tempfile::tempdir_in(env::current_dir().unwrap()).unwrap();
    let args = [
        "",
        "--target=aurix",
        "--tracing",
        xml_path,
        workspace_folder.path().to_str().unwrap(),
        "--workspace-generation",
    ];
    main_parse_arguments(args);

    // Enable the aggregate `all` feature plus exactly one CSFR cpu, and add the
    // `[[bin]]` target for the bundled main.rs. The `tracing` feature is left
    // disabled (tracing code is generated but gated off). `csfr_cpuN` is
    // intentionally not part of `all`, so it must be selected explicitly.
    patch_root_cargo_toml(workspace_folder.path(), &["all", "csfr_cpu0"]);

    // Add the `tc162-rt` path dependency (used by the bundled main.rs) and
    // register it as a workspace member. The copied resources contain no
    // top-level Cargo.toml and `tc162-rt/Cargo.toml` declares no `[workspace]`,
    // so member-patching does not conflict with the generated workspace root.
    let cargo_path = workspace_folder.path().join("Cargo.toml");
    let old_toml = fs::read_to_string(&cargo_path).expect("Unable to read toml file");
    let mut parsed_toml = old_toml
        .parse::<Document>()
        .expect("Unable to parse toml file");
    parsed_toml["dependencies"]["tc162-rt"]["path"] = value("tc162-rt");
    parsed_toml["workspace"]["members"]
        .as_array_mut()
        .expect("workspace.members is not an array")
        .push("tc162-rt");
    fs::write(&cargo_path, parsed_toml.to_string()).expect("Unable to write toml file");

    // Copy the aurix project files: main.rs at src/bin, tc162-rt runtime,
    // linker-scripts and the top-level .cargo/config.toml (sets the build
    // target for the whole workspace).
    fs_extra::dir::copy(
        "./tests/resources/project_files_aurix",
        workspace_folder.path(),
        &CopyOptions::new().content_only(true).overwrite(true),
    )
    .expect("Failed to copy required files to build cargo project");

    let license_path = workspace_folder.path().join("LICENSE.txt");
    assert!(license_path.exists(), "Not found LICENSE.txt");

    assert_cargo_command(
        &workspace_folder,
        CargoCommand::Build,
        Some(env!("AURIX_TOOLCHAIN").to_string()),
    );
    assert_cargo_command(
        &workspace_folder,
        CargoCommand::Clean,
        Some(env!("AURIX_TOOLCHAIN").to_string()),
    );
}
