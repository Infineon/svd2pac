#![cfg(not(aurix_tests))]
mod common;
use common::*;
use fs_extra::dir::CopyOptions;
use std::env;
use svd2pac::main;

/// Test generic target code generation.
#[test]
fn compile_generated_cortex_m() {
    let xml_path = concat!(env!("CARGO_MANIFEST_DIR"), "/test_svd/simple.xml");

    // Temp folder that should be deleted in case of test success.
    let workspace_folder = tempfile::tempdir_in(env::current_dir().unwrap()).unwrap();
    fs_extra::dir::copy(
        "./tests/resources/project_files_cortex_m",
        workspace_folder.path(),
        &CopyOptions::new().content_only(true).overwrite(true),
    )
    .expect("Failed to copy required files to build cargo project");

    let generated_pack_folder = workspace_folder.path().join("arm_example");
    let args = [
        "",
        "--target=cortex-m",
        xml_path,
        generated_pack_folder.to_str().unwrap(),
    ];
    main(args);

    let license_path = generated_pack_folder.join("LICENSE.txt");
    assert!(license_path.exists(), "Not found LICENSE.txt");
    assert_cargo_build(workspace_folder);
}
