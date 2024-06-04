#![cfg(aurix_tests)]
mod common;
use common::*;
use fs_extra::dir::CopyOptions;
use std::fs;
use std::{env, path::Path};
use svd2pac::main_parse_arguments;
use toml_edit::{array, value, Array, Document, Table};

/// This test requires Aurix Rust compiler pre-installed
#[test]
fn compile_generated_aurix() {
    let xml_path = concat!(env!("CARGO_MANIFEST_DIR"), "/test_svd/simple.xml");

    // Temp folder that should be deleted in case of test success.
    let generated_code_folder = tempfile::tempdir_in(env::current_dir().unwrap()).unwrap();
    let args = [
        "",
        "--disable-rust-fmt",
        "--target=aurix",
        xml_path,
        generated_code_folder.path().to_str().unwrap(),
    ];
    main_parse_arguments(args);
    //Patch toml and add required files.
    let old_toml = fs::read_to_string(Path::new(&generated_code_folder.path().join("Cargo.toml")))
        .expect("Unable to read toml file");
    let mut parsed_toml = old_toml
        .parse::<Document>()
        .expect("Unable to parse toml file");
    let mut default_table = Array::new();
    default_table.push("all");
    default_table.push("csfr_cpu0");
    parsed_toml["features"]["default"] = value(default_table);
    parsed_toml["dependencies"]["tc162-rt"]["path"] = value("tc162-rt");
    parsed_toml["bin"] = array();
    let bin_array = parsed_toml["bin"].as_array_of_tables_mut().unwrap();

    let mut bin_table = Table::new();
    bin_table["name"] = value("main");
    bin_table["test"] = value(false);
    bin_table["bench"] = value(false);
    bin_array.push(bin_table);
    let toml_string = parsed_toml.to_string();
    print!("{}", toml_string);
    // Write to a file
    fs::write(
        Path::new(Path::new(&generated_code_folder.path().join("Cargo.toml"))),
        toml_string,
    )
    .expect("Unable to write toml file");

    fs_extra::dir::copy(
        "./tests/resources/project_files_aurix",
        generated_code_folder.path(),
        &CopyOptions::new().content_only(true).overwrite(true),
    )
    .expect("Failed to copy required files to build cargo project");

    assert_cargo_build(generated_code_folder);
}

/// Generate PAC with tracing code but feature is disabled
#[test]
fn compile_generated_aurix_tracing() {
    let xml_path = concat!(env!("CARGO_MANIFEST_DIR"), "/test_svd/simple.xml");

    // Temp folder that should be deleted in case of test success.
    let generated_code_folder = tempfile::tempdir_in(env::current_dir().unwrap()).unwrap();
    let args = [
        "",
        "--disable-rust-fmt",
        "--target=aurix",
        "--tracing",
        xml_path,
        generated_code_folder.path().to_str().unwrap(),
    ];
    main_parse_arguments(args);

    //Patch toml and add required files.
    let old_toml = fs::read_to_string(Path::new(&generated_code_folder.path().join("Cargo.toml")))
        .expect("Unable to read toml file");
    let mut parsed_toml = old_toml
        .parse::<Document>()
        .expect("Unable to parse toml file");
    let mut default_table = Array::new();
    default_table.push("all");
    default_table.push("csfr_cpu0");
    parsed_toml["features"]["default"] = value(default_table);
    parsed_toml["dependencies"]["tc162-rt"]["path"] = value("tc162-rt");
    parsed_toml["bin"] = array();
    let bin_array = parsed_toml["bin"].as_array_of_tables_mut().unwrap();

    let mut bin_table = Table::new();
    bin_table["name"] = value("main");
    bin_table["test"] = value(false);
    bin_table["bench"] = value(false);
    bin_array.push(bin_table);
    let toml_string = parsed_toml.to_string();
    print!("{}", toml_string);
    // Write to a file
    fs::write(
        Path::new(Path::new(&generated_code_folder.path().join("Cargo.toml"))),
        toml_string,
    )
    .expect("Unable to write toml file");

    fs_extra::dir::copy(
        "./tests/resources/project_files_aurix",
        generated_code_folder.path(),
        &CopyOptions::new().content_only(true).overwrite(true),
    )
    .expect("Failed to copy required files to build cargo project");

    assert_cargo_build(generated_code_folder);
}
