mod ir;
mod util;
mod xml2ir;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use self::util::ToSanitizedSymbol;
use crate::{SvdValidationLevel, Target};
use anyhow::{Context, Result, anyhow};
use lazy_regex::regex;
use log::{error, info, warn};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::process::Command;
use tera::{Tera, Value, to_value, try_get_value};

/// Convert [`Vec<PathChunk>`] to a string representation of a register path.
fn filter_render_path(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    match serde_json::from_value::<Vec<ir::PathChunk>>(value.clone()) {
        Ok(path) => {
            let rendered =
                path.iter()
                    .enumerate()
                    .fold(String::new(), |mut output, (index, path_chunk)| {
                        let _ = write!(
                            output,
                            "{}{}{}",
                            match index {
                                // check first element
                                0 => path_chunk.path.clone(),
                                _ => path_chunk.path.clone() + "()",
                            },
                            path_chunk
                                .index
                                .map_or_else(String::new, |index| format!("[{index}]")),
                            match index {
                                i if (i == path.len() - 1) => String::default(),
                                _ => ".".to_owned(),
                            }
                        );
                        output
                    });
            Ok(Value::String(rendered))
        }
        Err(e) => Err(tera::Error::msg(format!(
            "filter_render_path only accepts Vec<PathChunk> as input.\nCannot deserialize value:{value} because:\nerror:{e}"
        ))),
    }
}

/// Convert a String to array of line splitting it at \n or \r\n and prepend each line with `prefix`
/// This filter is intended to create multiple line comments from a String
fn filter_prepend_lines(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    let reg_ex = regex!(r#"(\r\n)|(\n)"#);
    let input_string = try_get_value!("prepend_lines", "value", String, value.clone());
    let prefix_string = match args.get("prefix") {
        Some(val) => try_get_value!("prepend_lines", "prefix", String, val),
        None => {
            return Err(tera::Error::msg(
                "Filter `prepend_lines` expected an arg called `prefix`",
            ));
        }
    };
    let splits = to_value(
        reg_ex
            .split(&input_string)
            .map(|s| prefix_string.clone() + s)
            .collect::<Vec<_>>(),
    )
    .map_err(|e| tera::Error::msg(format!("Failed to convert to value: {e}")))?;
    Ok(splits)
}

/// Convert JSON number to hexadecimal String filter for tera template
fn filter_to_hex(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    if let Value::Number(number) = value {
        number.as_u64().map_or_else(
            || Err(tera::Error::msg("to_hex accept only unsigned numbers")),
            |u64_val| Ok(Value::String(format!("0x{u64_val:x}"))),
        )
    } else {
        Err(tera::Error::msg(format!(
            "to_hex accept only numbers as input. value:{value}"
        )))
    }
}

/// Convert stringified number to hex value. Useful when iterating over maps
/// in tera files as tera converts the key to strings regardless of the type.
fn filter_num_str_to_hex(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    if let Value::String(number_str) = value {
        number_str.parse::<u64>().map_or_else(
            |_| {
                Err(tera::Error::msg(format!(
                    "num_str_to_hex could not parse value:{value} as number"
                )))
            },
            |u64_val| Ok(Value::String(format!("0x{u64_val:x}"))),
        )
    } else {
        Err(tera::Error::msg(format!(
            "num_str_to_hex only accepts strings. value:{value}"
        )))
    }
}

fn filter_to_struct_id(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    if let Value::String(string) = value {
        Ok(Value::String(string.to_owned().to_sanitized_struct_ident()))
    } else {
        Err(tera::Error::msg(format!(
            "filter_to_struct_id only supports String as argument. value:{value}"
        )))
    }
}

fn filter_to_mod_id(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    if let Value::String(string) = value {
        Ok(Value::String(string.to_owned().to_sanitized_mod_ident()))
    } else {
        Err(tera::Error::msg(format!(
            "filter_to_mod_id only supports String as argument. value:{value}",
        )))
    }
}

#[allow(dead_code)]
fn filter_to_enum_id(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    if let Value::String(string) = value {
        Ok(Value::String(string.to_owned().to_sanitized_enum_ident()))
    } else {
        Err(tera::Error::msg(format!(
            "filter_to_enum_id case support only String as argument. value:{value}",
        )))
    }
}

fn filter_to_const_id(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    if let Value::String(string) = value {
        Ok(Value::String(string.to_owned().to_sanitized_const_ident()))
    } else {
        Err(tera::Error::msg(format!(
            "filter_to_const_id only supports String as argument. value:{value}",
        )))
    }
}

fn filter_to_func_id(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    if let Value::String(string) = value {
        Ok(Value::String(string.to_owned().to_sanitized_func_ident()))
    } else {
        Err(tera::Error::msg(format!(
            "filter_to_func_id only supports String as argument. value:{value}"
        )))
    }
}

/// Sanitize a string so it can be used in doc attribute
fn filter_svd_description_to_doc(
    value: &Value,
    _args: &HashMap<String, Value>,
) -> tera::Result<Value> {
    if let Value::String(doc_string) = value {
        Ok(Value::String(
            doc_string
                .replace('[', r"\[")
                .replace(']', r"\]")
                .escape_debug()
                .to_string(),
        ))
    } else {
        Err(tera::Error::msg(
            "svd_description_to_doc accepts only string",
        ))
    }
}

fn execute_template(
    tera: &Tera,
    template_name: &str,
    context: &tera::Context,
    output_path: &Path,
) -> anyhow::Result<()> {
    let result = match tera.render(template_name, context) {
        Ok(s) => Ok(s),
        Err(e) => {
            error!("Render Error: {e}");
            let mut cause = e.source();
            while let Some(e) = cause {
                error!("Render Reason: {e}");
                cause = e.source();
            }
            Err(anyhow!("Failed to render"))
        }
    }?;
    let folder = output_path
        .parent()
        .unwrap_or_else(|| panic!("No parent folder for {}", output_path.display()));
    create_dir_all(folder)?;
    fs::write(output_path, result)
        .context(format!("Error while writing {}", output_path.display()))?;
    Ok(())
}

/// Get instance of Tera that includes required templates
fn get_tera_instance() -> anyhow::Result<Tera> {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        (
            "Cargo_toml.tera",
            include_str!("../templates/rust/Cargo_toml.tera"),
        ),
        ("lib.tera", include_str!("../templates/rust/lib.tera")),
        (
            "tracing.tera",
            include_str!("../templates/rust/tracing.tera"),
        ),
        (
            "reg_name.tera",
            include_str!("../templates/rust/reg_name.tera"),
        ),
        (
            "peri_mod.tera",
            include_str!("../templates/rust/peri_mod.tera"),
        ),
        (
            "aurix_core.tera",
            include_str!("../templates/rust/aurix_core.tera"),
        ),
        ("common.tera", include_str!("../templates/rust/common.tera")),
        ("macros.tera", include_str!("../templates/rust/macros.tera")),
        (
            "build_cortex.tera",
            include_str!("../templates/rust/build_cortex.tera"),
        ),
        (
            "device_x.tera",
            include_str!("../templates/rust/device_x.tera"),
        ),
        (
            "workspace_Cargo_toml.tera",
            include_str!("../templates/rust/workspace/workspace_Cargo_toml.tera"),
        ),
        (
            "workspace_lib.tera",
            include_str!("../templates/rust/workspace/workspace_lib.tera"),
        ),
        (
            "peri_Cargo_toml.tera",
            include_str!("../templates/rust/workspace/peri_Cargo_toml.tera"),
        ),
        (
            "peri_lib.tera",
            include_str!("../templates/rust/workspace/peri_lib.tera"),
        ),
        (
            "common_Cargo_toml.tera",
            include_str!("../templates/rust/workspace/common_Cargo_toml.tera"),
        ),
    ])?;
    Ok(tera)
}

fn get_xml_string(path: &Path, xml: &mut String) -> Result<()> {
    File::open(path)
        .context("Cannot open register description file")?
        .read_to_string(xml)
        .context("Cannot read register description file")?;
    Ok(())
}

// check if vendor extension is present in the svd
fn check_for_vendor_extension(path: &Path) -> Result<bool> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut present = false;
    for line in reader.lines() {
        match line? {
            s if s.contains("vendorExtensions") => present = true,
            _ => {}
        }
    }
    Ok(present)
}

fn error_with_context() -> Result<()> {
    Err(anyhow!("svd parsing error")).context("problem with svd to extract aurix cpu related peripherals, <vendorExtensions> or <aurixCSFR> maybe missing?")
}

/// Extracts the AURIX CSFR SVD section from the given XML file and constructs a new SVD string.
///
/// # Arguments
///
/// * `path` - The path to the XML file containing the SVD.
/// * `svd_string` - A mutable reference to a string where the extracted SVD will be stored.
///
/// # Returns
///
/// * `Result<()>` - Returns an Ok(()) if successful, or an error if there was a problem reading the file or parsing the XML.
pub fn get_aurix_csfr_svd(path: &Path, svd_string: &mut String) -> Result<()> {
    let xml = &mut String::new();
    File::open(path)
        .context("Cannot open register description file")?
        .read_to_string(xml)
        .context("Cannot read register description file")?;

    let common_svd_tags: Vec<&str> = xml.split("<peripherals>").collect();
    let svd_split_vec: Vec<&str> = xml.split("<aurixCSFR>").collect();
    let extended_peripherals: Vec<&str> = if svd_split_vec.len() > 1 {
        svd_split_vec[1].split("</aurixCSFR>").collect()
    } else {
        error_with_context()?;
        Vec::new()
    };
    *svd_string = format!(
        "{} {} {} {} {}",
        common_svd_tags[0],
        "<peripherals>",
        extended_peripherals[0],
        "</peripherals>\n",
        "</device>"
    );
    Ok(())
}

pub struct GenPkgSettings {
    pub run_rustfmt: bool,
    pub svd_validation_level: SvdValidationLevel,
    pub target: Target,
    pub tracing: bool,
    pub package_name: Option<String>,
    pub license_file: Option<PathBuf>,
    pub svd2pac_version: String,
    pub workspace_generation: bool,
}

fn precompile_tera(tera: &mut Tera) {
    tera.register_filter("to_hex", filter_to_hex);
    tera.register_filter("num_str_to_hex", filter_num_str_to_hex);
    tera.register_filter("render_path", filter_render_path);
    tera.register_filter("to_struct_id", filter_to_struct_id);
    tera.register_filter("to_func_id", filter_to_func_id);
    tera.register_filter("to_mod_id", filter_to_mod_id);
    tera.register_filter("to_enumerated_const_id", filter_to_const_id);
    tera.register_filter("prepend_lines", filter_prepend_lines);
    tera.register_filter("svd_description_to_doc", filter_svd_description_to_doc);
}

fn generate_lib_rs_module(
    tera: &Tera,
    destination_folder: &Path,
    context: &tera::Context,
) -> anyhow::Result<()> {
    let lib_path = destination_folder.join("src/lib.rs");
    execute_template(tera, "lib.tera", context, &lib_path).context("Failed generation of code")?;
    Ok(())
}

fn generate_cargo_toml(
    tera: &Tera,
    destination_folder: &Path,
    context: &tera::Context,
) -> anyhow::Result<()> {
    execute_template(
        tera,
        "Cargo_toml.tera",
        context,
        &destination_folder.join("Cargo.toml"),
    )
    .context("Failed generation of Cargo.toml")?;
    Ok(())
}

fn generate_tracing_module(
    tera: &Tera,
    ir: &ir::IR,
    destination_folder: &Path,
    context: &tera::Context,
) -> anyhow::Result<()> {
    // tracing module
    let lib_path = destination_folder.join("src/tracing.rs");
    execute_template(tera, "tracing.tera", context, &lib_path)
        .context("Failed generation of tracing.rs")?;

    let svd2pac_version = context.get("svd2pac_version").unwrap().as_str();
    let now = context.get("now").unwrap().as_str();
    // reg_name module
    //
    // # Issue
    //
    // When using `feature=[tracing]`, the exact name of accesses registers
    // is lost, due to registers being accessed though their raw address.
    // When writing test this is sufficient as accesses happen to the same
    // addresses as on embedded anyway (regardless if registers are aliased
    // or not).
    // When evaluating failed tests however, having a human readable name
    // of a register is extremely valuable.
    //
    // # Solution
    //
    // When tracing is enabled, we generate a hash map of all physical
    // addresses to a string name of registers at a specific address.
    //
    // Due to SVD supporting aliasing (i.e. the same physical address can be
    // reused by multiple different peripherals, clusters and registers),
    // the string name contains **all** possible aliases that resolve to
    // a specific address.
    //
    // # How
    //
    // A [`phf`](https://crates.io/crate/phf) of physical addresses to
    // associated register names is generated as `reg_name.rs` module.
    //
    // This is done by flattening the SVD-tree structure into a map
    // of `Vec<PathChunk>` to `RegisterAbs`, which is then inverted to
    // a map of `address` to Vec<Vec<PathChunk>>.
    //
    // This map is passed to the tera module where it is rendered
    // into the respective hash map entries.
    let mut context = tera::Context::new();
    let lib_path = destination_folder.join("src/reg_name.rs");
    context.insert("register_addresses", &ir.register_addresses);
    context.insert("ir", &ir);
    context.insert("svd2pac_version", &svd2pac_version);
    context.insert("now", &now);
    execute_template(tera, "reg_name.tera", &context, &lib_path)
        .context("Failed generation of reg_name.rs")?;
    Ok(())
}

fn generate_common_module(
    tera: &Tera,
    _ir: &ir::IR,
    destination_folder: &Path,
    context: &tera::Context,
) -> anyhow::Result<()> {
    let lib_path = destination_folder.join("src/common.rs");
    execute_template(tera, "common.tera", context, &lib_path)
        .context("Failed generation of common.rs")?;
    Ok(())
}

fn generate_peripheral_module(
    tera: &Tera,
    ir: &ir::IR,
    template_name: &str,
    destination_folder: &Path,
    svd2pac_version: &str,
    now: &str,
) -> anyhow::Result<()> {
    // Generate one module for each peripheral
    for (_, peri) in &ir.device.peripheral_mod {
        // No need to generate a module if the peripheral is derived
        let borrowed_peri = peri.borrow();
        if borrowed_peri.derived_from.is_some() {
            continue;
        }
        let module_name = borrowed_peri.module_id.clone();
        let mut context = tera::Context::new();
        context.insert("peri", peri);
        context.insert("ir", &ir);
        context.insert("svd2pac_version", svd2pac_version);
        context.insert("now", now);
        execute_template(
            tera,
            template_name,
            &context,
            &destination_folder.join(format!("src/{module_name}.rs")),
        )
        .context("Failed generation of code")?;
    }
    Ok(())
}

fn read_license_file(license_file: Option<&PathBuf>) -> anyhow::Result<Option<String>> {
    license_file
        .map(|path| {
            fs::read_to_string(path)
                .with_context(|| format!("Unable to read license file {}", path.display()))
        })
        .transpose()
}

fn load_ir(
    xml_path: &Path,
    svd_validation_level: SvdValidationLevel,
    custom_license_text: Option<&String>,
) -> anyhow::Result<ir::IR> {
    let xml = &mut String::new();
    get_xml_string(xml_path, xml)?;
    let svd_device = xml2ir::parse_xml(xml, svd_validation_level)?;
    xml2ir::svd_device2ir(&svd_device, custom_license_text)
}

fn generate_cortex_m_extra_files(
    tera: &Tera,
    context: &tera::Context,
    destination_folder: &Path,
) -> anyhow::Result<()> {
    execute_template(
        tera,
        "device_x.tera",
        context,
        &destination_folder.join("device.x"),
    )
    .context("Failed to generate device.x file")?;
    execute_template(
        tera,
        "build_cortex.tera",
        context,
        &destination_folder.join("build.rs"),
    )
    .context("Failed to generate build.rs file")?;
    Ok(())
}

fn format_with_rustfmt(files: &[PathBuf]) -> anyhow::Result<()> {
    match Command::new("rustfmt").arg("-V").output() {
        Ok(_) => {
            info!("Formatting code with rustfmt");
            for file in files {
                Command::new("rustfmt").arg(file).status()?;
            }
        }
        Err(_) => {
            warn!(
                "Error while detecting presence of rustfmt. Generated code is valid but not formatted"
            );
        }
    }
    Ok(())
}

fn generate_aurix_core_ir(
    xml_path: &Path,
    settings: &GenPkgSettings,
) -> anyhow::Result<Option<ir::IR>> {
    let GenPkgSettings {
        run_rustfmt: _,
        svd_validation_level,
        target: _,
        tracing: _,
        package_name: _,
        license_file,
        svd2pac_version: _,
        workspace_generation: _,
    } = settings;

    info!("Start generating csfr rust code");
    // Read license file if specified
    let custom_license_text = read_license_file(license_file.as_ref())?;
    // If target is aurix, create csfr
    let result = check_for_vendor_extension(xml_path)?;
    if result {
        let svd_csfr_xml = &mut String::with_capacity(500);
        get_aurix_csfr_svd(xml_path, svd_csfr_xml)?;
        let mut svd_device = xml2ir::parse_xml(svd_csfr_xml, *svd_validation_level)?;
        // Rename peripherals
        for peri in &mut svd_device.peripherals {
            peri.name = "csfr_".to_string() + &peri.name;
        }
        let ir_csfr = xml2ir::svd_device2ir(&svd_device, custom_license_text.as_ref())?;
        Ok(Some(ir_csfr))
    } else {
        Ok(None)
    }
}

pub fn generate_rust_package(
    xml_path: &Path,
    destination_folder: &Path,
    settings: &GenPkgSettings,
) -> anyhow::Result<()> {
    let GenPkgSettings {
        run_rustfmt,
        svd_validation_level,
        target,
        tracing,
        ref package_name,
        ref license_file,
        ref svd2pac_version,
        workspace_generation,
    } = *settings;

    // Route to workspace generation when requested.
    if workspace_generation {
        return generate_rust_workspace(xml_path, destination_folder, settings);
    }

    info!("Start generating rust code");
    // Read license file if specified
    let custom_license_text = read_license_file(license_file.as_ref())?;
    let ir = load_ir(xml_path, svd_validation_level, custom_license_text.as_ref())?;
    //Precompile templates
    let mut tera = get_tera_instance()?;
    precompile_tera(&mut tera);

    let package_name: String = match package_name {
        None => ir.device.name.to_lowercase(),
        Some(package_name) => package_name.clone(),
    };

    let now = chrono::Utc::now().to_rfc2822();

    let mut context = tera::Context::new();
    context.insert("ir", &ir);
    context.insert("target", &target);
    context.insert("tracing", &tracing);
    context.insert("package_name", &package_name);
    context.insert("description", "Description tests");
    context.insert("svd2pac_version", svd2pac_version);
    context.insert("now", &now);
    context.insert("workspace_mode", &false);

    // Generate peripheral modules
    generate_peripheral_module(
        &tera,
        &ir,
        "peri_mod.tera",
        destination_folder,
        svd2pac_version,
        &now,
    )?;

    //Generate common module
    generate_common_module(&tera, &ir, destination_folder, &context)?;

    // Generate tracing related modules
    if tracing {
        generate_tracing_module(&tera, &ir, destination_folder, &context)?;
    }

    // If target is aurix, create csfr modules
    if settings.target == Target::Aurix {
        let ir_csfr = generate_aurix_core_ir(xml_path, settings)?;

        // Generate cpu peripheral modules
        if let Some(ref ir) = ir_csfr {
            generate_peripheral_module(
                &tera,
                ir,
                "aurix_core.tera",
                destination_folder,
                svd2pac_version,
                &now,
            )?;
            context.insert("ir_csfr", &ir_csfr);
        }
    }

    //generate lib.rs
    generate_lib_rs_module(&tera, destination_folder, &context)?;

    //generate Cargo.toml
    generate_cargo_toml(&tera, destination_folder, &context)?;

    // If cortex-m add build.rs and device.x
    if settings.target == Target::CortexM {
        generate_cortex_m_extra_files(&tera, &context, destination_folder)?;
    }

    // Run rustfmt on generated code
    if run_rustfmt {
        format_with_rustfmt(&[destination_folder.join("src/lib.rs")])?;
    }
    // Add license file
    fs::write(destination_folder.join("LICENSE.txt"), ir.license_text)?;

    info!("Completed code generation");
    Ok(())
}

/// Collect the set of *other* peripheral module ids whose types are referenced
/// by the registers/clusters of a peripheral.
///
/// A peripheral references another one when it has a register or cluster that is
/// derived from (or otherwise shares the struct module of) a peripheral with a
/// different `module_id`. In workspace mode these references map to a path
/// dependency on the corresponding peripheral crate.
fn collect_referenced_modules(
    peri: &ir::PeripheralMod,
    own_module_id: &str,
    acc: &mut std::collections::BTreeSet<String>,
) {
    fn visit_cluster(
        cluster: &ir::Cluster,
        own_module_id: &str,
        acc: &mut std::collections::BTreeSet<String>,
    ) {
        if let Some(first) = cluster.struct_module_path.first() {
            if first != own_module_id {
                acc.insert(first.clone());
            }
        }
        for reg in cluster.registers.values() {
            if let Some(first) = reg.borrow().struct_module_path.first() {
                if first != own_module_id {
                    acc.insert(first.clone());
                }
            }
        }
        for nested in cluster.clusters.values() {
            visit_cluster(&nested.borrow(), own_module_id, acc);
        }
    }

    for reg in peri.registers.values() {
        if let Some(first) = reg.borrow().struct_module_path.first() {
            if first != own_module_id {
                acc.insert(first.clone());
            }
        }
    }
    for cluster in peri.clusters.values() {
        visit_cluster(&cluster.borrow(), own_module_id, acc);
    }
}

/// Generate the per-peripheral crates for a Cargo workspace.
///
/// For each non-derived peripheral a crate is created under
/// `destination_folder/<module_id>/` containing `src/<module_id>.rs`,
/// `src/lib.rs`, and `Cargo.toml`.
///
/// Returns the paths of the generated Rust source files so the caller can
/// pass them to `rustfmt`.
fn generate_workspace_peripheral_crates(
    tera: &Tera,
    ir: &ir::IR,
    context: &tera::Context,
    destination_folder: &Path,
) -> anyhow::Result<Vec<PathBuf>> {
    let mut rust_files: Vec<PathBuf> = Vec::new();
    for (_, peri) in &ir.device.peripheral_mod {
        // Derived peripherals do not own a crate; their constants are emitted in
        // the root crate.
        if peri.borrow().derived_from.is_some() {
            continue;
        }
        let module_id = peri.borrow().module_id.clone();
        let peri_dir = destination_folder.join(&module_id);

        // Collect cross-peripheral references so they can be added as path
        // dependencies of this peripheral crate.
        let mut referenced = std::collections::BTreeSet::new();
        collect_referenced_modules(&peri.borrow(), &module_id, &mut referenced);
        let referenced_crates: Vec<String> = referenced.into_iter().collect();

        let mut peri_context = context.clone();
        peri_context.insert("peri", peri);
        peri_context.insert("referenced_crates", &referenced_crates);

        // Peripheral module (existing template, rendered unchanged).
        let peri_mod_path = peri_dir.join(format!("src/{module_id}.rs"));
        execute_template(tera, "peri_mod.tera", &peri_context, &peri_mod_path)
            .context("Failed generation of peripheral module")?;
        rust_files.push(peri_mod_path);

        // Peripheral crate lib.rs (struct + extern crate common + module + constants).
        let peri_lib_path = peri_dir.join("src/lib.rs");
        execute_template(tera, "peri_lib.tera", &peri_context, &peri_lib_path)
            .context("Failed generation of peripheral lib.rs")?;
        rust_files.push(peri_lib_path);

        // Peripheral crate Cargo.toml.
        execute_template(
            tera,
            "peri_Cargo_toml.tera",
            &peri_context,
            &peri_dir.join("Cargo.toml"),
        )
        .context("Failed generation of peripheral Cargo.toml")?;
    }
    Ok(rust_files)
}

/// Generate a Cargo workspace instead of a single package.
///
/// Each non-derived peripheral becomes its own crate, with a shared `common`
/// crate and a top-level root crate that re-exports everything behind feature
/// gates.
#[allow(clippy::too_many_lines)]
fn generate_rust_workspace(
    xml_path: &Path,
    destination_folder: &Path,
    settings: &GenPkgSettings,
) -> anyhow::Result<()> {
    let GenPkgSettings {
        run_rustfmt,
        svd_validation_level,
        target,
        tracing,
        ref package_name,
        ref license_file,
        ref svd2pac_version,
        workspace_generation: _,
    } = *settings;

    info!("Start generating rust workspace");

    // Read license file if specified
    let custom_license_text = read_license_file(license_file.as_ref())?;
    let ir = load_ir(xml_path, svd_validation_level, custom_license_text.as_ref())?;

    // Precompile templates
    let mut tera = get_tera_instance()?;
    precompile_tera(&mut tera);

    let package_name: String = match package_name {
        None => ir.device.name.to_lowercase(),
        Some(package_name) => package_name.clone(),
    };

    let now = chrono::Utc::now().to_rfc2822();

    // Base context shared by all workspace templates.
    let mut context = tera::Context::new();
    context.insert("ir", &ir);
    context.insert("target", &target);
    context.insert("tracing", &tracing);
    context.insert("package_name", &package_name);
    context.insert("description", "Description tests");
    context.insert("svd2pac_version", svd2pac_version);
    context.insert("now", &now);
    context.insert("workspace_mode", &true);

    // Collect generated rust files so they can be formatted at the end.
    let mut rust_files: Vec<PathBuf> = Vec::new();

    // --- common crate ---
    let common_dir = destination_folder.join("common");
    let common_lib = common_dir.join("src/lib.rs");
    execute_template(&tera, "common.tera", &context, &common_lib)
        .context("Failed generation of common/src/lib.rs")?;
    rust_files.push(common_lib);
    execute_template(
        &tera,
        "common_Cargo_toml.tera",
        &context,
        &common_dir.join("Cargo.toml"),
    )
    .context("Failed generation of common/Cargo.toml")?;
    if tracing {
        let tracing_path = common_dir.join("src/tracing.rs");
        execute_template(&tera, "tracing.tera", &context, &tracing_path)
            .context("Failed generation of common/src/tracing.rs")?;
        rust_files.push(tracing_path);
    }

    // --- per-peripheral crates (non-derived peripherals only) ---
    rust_files.extend(generate_workspace_peripheral_crates(
        &tera,
        &ir,
        &context,
        destination_folder,
    )?);

    // --- root crate ---
    // Aurix CSFR: generate the core register modules inside the root crate,
    // mirroring single-package mode where they live next to lib.rs. Returns
    // `None` when the SVD has no vendor extensions, so the plain-aurix path is
    // unaffected.
    if target == Target::Aurix {
        if let Some(ir_csfr) = generate_aurix_core_ir(xml_path, settings)? {
            generate_peripheral_module(
                &tera,
                &ir_csfr,
                "aurix_core.tera",
                destination_folder,
                svd2pac_version,
                &now,
            )?;
            for (_, peri) in &ir_csfr.device.peripheral_mod {
                let borrowed_peri = peri.borrow();
                if borrowed_peri.derived_from.is_some() {
                    continue;
                }
                rust_files
                    .push(destination_folder.join(format!("src/{}.rs", borrowed_peri.module_id)));
            }
            context.insert("ir_csfr", &ir_csfr);
        }
    }

    let root_lib = destination_folder.join("src/lib.rs");
    execute_template(&tera, "workspace_lib.tera", &context, &root_lib)
        .context("Failed generation of root src/lib.rs")?;
    rust_files.push(root_lib);
    execute_template(
        &tera,
        "workspace_Cargo_toml.tera",
        &context,
        &destination_folder.join("Cargo.toml"),
    )
    .context("Failed generation of root Cargo.toml")?;

    // Root reg_name.rs (uses phf directly) when tracing is enabled.
    if tracing {
        let mut reg_context = tera::Context::new();
        reg_context.insert("register_addresses", &ir.register_addresses);
        reg_context.insert("ir", &ir);
        reg_context.insert("svd2pac_version", svd2pac_version);
        reg_context.insert("now", &now);
        let reg_name_path = destination_folder.join("src/reg_name.rs");
        execute_template(&tera, "reg_name.tera", &reg_context, &reg_name_path)
            .context("Failed generation of reg_name.rs")?;
        rust_files.push(reg_name_path);
    }

    // Cortex-M needs device.x and build.rs at the workspace root.
    if target == Target::CortexM {
        generate_cortex_m_extra_files(&tera, &context, destination_folder)?;
    }

    // Format generated rust code.
    if run_rustfmt {
        format_with_rustfmt(&rust_files)?;
    }

    // Add license file at the workspace root.
    fs::write(destination_folder.join("LICENSE.txt"), &ir.license_text)?;
    info!("Completed workspace code generation");
    Ok(())
}
