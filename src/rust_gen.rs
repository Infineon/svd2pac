mod ir;
mod util;
mod xml2ir;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use self::util::ToSanitizedSymbol;
use crate::{SvdValidationLevel, Target};
use anyhow::{anyhow, Context, Result};
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
use tera::{to_value, try_get_value, Tera, Value};

/// Convert [`Vec<PathChunk>`] to a string representation of a register path.
fn filter_render_path(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    match serde_json::from_value::<Vec<ir::PathChunk>>(value.clone()) {
        Ok(path) => {
            let rendered = path.iter()
                .enumerate()
                .fold(String::new(),|mut output,(index, path_chunk)|{
                    let _ = write!(output,
                        "{}{}{}",
                        match index {
                            // check first element
                            0 => path_chunk.path.clone(),
                            _ => path_chunk.path.clone() +"()",
                        },
                        match path_chunk.index {
                            Some(index) => format!("[{}]",index),
                            None => "".to_owned(),
                        },
                        match index {
                            _i if (_i == path.len() -1) => String::default(),
                            _ => ".".to_owned(),
                        }
                    );
                    output

                }
                );
            Ok(Value::String(rendered))
        },
        Err(e)=>{
            Err(tera::Error::msg(format!(
                "filter_render_path only acceptes Vec<PathChunk> as input.\nCound not deserialize value:{} because:\nerror:{}",
                value,
                e
            )))
        }
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
            ))
        }
    };
    let splits = to_value(
        reg_ex
            .split(&input_string)
            .map(|s| prefix_string.clone() + s)
            .collect::<Vec<_>>(),
    )
    .map_err(|e| tera::Error::msg(format!("Failed to convert to value: {}", e)))?;
    Ok(splits)
}

/// Convert JSON number to hexadecimal String filter for tera template
fn filter_to_hex(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    if let Value::Number(number) = value {
        if let Some(u64_val) = number.as_u64() {
            Ok(Value::String(format!("0x{:x}", u64_val)))
        } else {
            Err(tera::Error::msg("to_hex accept only unsigned numbers"))
        }
    } else {
        Err(tera::Error::msg(format!(
            "to_hex accept only numbers as input. value:{}",
            value
        )))
    }
}

/// Convert stringified number to hex value. Useful when iterating over maps
/// in tera files as tera converts the key to strings regardless of the type.
fn filter_num_str_to_hex(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    if let Value::String(number_str) = value {
        if let Ok(u64_val) = number_str.parse::<u64>() {
            Ok(Value::String(format!("0x{:x}", u64_val)))
        } else {
            Err(tera::Error::msg(format!(
                "num_str_to_hex could not parse value:{} as number",
                value
            )))
        }
    } else {
        Err(tera::Error::msg(format!(
            "num_str_to_hex only accepts strings. value:{}",
            value
        )))
    }
}

fn filter_to_struct_id(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    if let Value::String(string) = value {
        Ok(Value::String(string.to_owned().to_sanitized_struct_ident()))
    } else {
        Err(tera::Error::msg(format!(
            "filter_to_struct_id only supports String as argument. value:{}",
            value
        )))
    }
}

fn filter_to_mod_id(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    if let Value::String(string) = value {
        Ok(Value::String(string.to_owned().to_sanitized_mod_ident()))
    } else {
        Err(tera::Error::msg(format!(
            "filter_to_mod_id only supports String as argument. value:{}",
            value,
        )))
    }
}

#[allow(dead_code)]
fn filter_to_enum_id(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    if let Value::String(string) = value {
        Ok(Value::String(string.to_owned().to_sanitized_enum_ident()))
    } else {
        Err(tera::Error::msg(format!(
            "filter_to_enum_id case support only String as argument. value:{}",
            value,
        )))
    }
}

fn filter_to_const_id(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    if let Value::String(string) = value {
        Ok(Value::String(string.to_owned().to_sanitized_const_ident()))
    } else {
        Err(tera::Error::msg(format!(
            "filter_to_const_id only supports String as argument. value:{}",
            value,
        )))
    }
}

fn filter_to_func_id(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    if let Value::String(string) = value {
        Ok(Value::String(string.to_owned().to_sanitized_func_ident()))
    } else {
        Err(tera::Error::msg(format!(
            "filter_to_func_id only supports String as argument. value:{}",
            value
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
            error!("Render Error: {}", e);
            let mut cause = e.source();
            while let Some(e) = cause {
                error!("Render Reason: {}", e);
                cause = e.source();
            }
            Err(anyhow!("Failed to render"))
        }
    }?;
    let folder = output_path
        .parent()
        .unwrap_or_else(|| panic!("No parent folder for {:?}", output_path));
    create_dir_all(folder)?;
    fs::write(output_path, result).context(format!("Error while writing {:?}", output_path))?;
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
            _ => continue,
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
    let mut extended_peripherals: Vec<&str> = Vec::new();
    if svd_split_vec.len() > 1 {
        extended_peripherals = svd_split_vec[1].split("</aurixCSFR>").collect();
    } else {
        error_with_context()?;
    }
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
            &destination_folder.join(format!("src/{}.rs", module_name)),
        )
        .context("Failed generation of code")?;
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
    } = settings;

    info!("Start generating csfr rust code");
    // Read license file if specified
    let custom_license_text = license_file.as_ref().map(|path| {
        fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read license file {path:?}"))
    });
    // If target is aurix, create csfr
    let result = check_for_vendor_extension(xml_path)?;
    if result {
        let svd_csfr_xml = &mut String::with_capacity(500);
        get_aurix_csfr_svd(xml_path, svd_csfr_xml)?;
        let mut svd_device = xml2ir::parse_xml(svd_csfr_xml, *svd_validation_level)?;
        // Rename peripherals
        for peri in svd_device.peripherals.iter_mut() {
            peri.name = "csfr_".to_string() + &peri.name
        }
        let ir_csfr = xml2ir::svd_device2ir(&svd_device, &custom_license_text)?;
        Ok(Some(ir_csfr))
    } else {
        Ok(None)
    }
}

pub(crate) fn generate_rust_package(
    xml_path: &Path,
    destination_folder: &Path,
    settings: GenPkgSettings,
) -> anyhow::Result<()> {
    let GenPkgSettings {
        run_rustfmt,
        svd_validation_level,
        target,
        tracing,
        ref package_name,
        ref license_file,
        ref svd2pac_version,
    } = settings;

    info!("Start generating rust code");
    // Read license file if specified
    let custom_license_text = license_file.as_ref().map(|path| {
        fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read license file {path:?}"))
    });

    let xml = &mut String::new();
    get_xml_string(xml_path, xml)?;
    let svd_device = xml2ir::parse_xml(xml, svd_validation_level)?;
    let ir = xml2ir::svd_device2ir(&svd_device, &custom_license_text)?;
    //Precompile templates
    let mut tera = get_tera_instance()?;
    precompile_tera(&mut tera);

    let package_name: String = match package_name {
        None => ir.device.name.clone().to_lowercase(),
        Some(ref package_name) => package_name.clone(),
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
        let ir_csfr = generate_aurix_core_ir(xml_path, &settings)?;

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
        execute_template(
            &tera,
            "device_x.tera",
            &context,
            &destination_folder.join("device.x"),
        )
        .context("Failed to generate device.x file")?;
        execute_template(
            &tera,
            "build_cortex.tera",
            &context,
            &destination_folder.join("build.rs"),
        )
        .context("Failed to generate build.rs file")?;
    }

    let lib_path = destination_folder.join("src/lib.rs");
    // Run rustfmt on generated code
    if run_rustfmt {
        // Check rustfmt is available
        match Command::new("rustfmt").arg("-V").output() {
            // If available format code and return
            Ok(_) => {
                info!("Formatting code with rustfmt");
                Command::new("rustfmt").arg(lib_path).status()?;
            }
            // if not able to run with --help proceed just with a warning. Generated code is anyway valid.
            Err(_) => {
                warn!("Error while detecting presence of rustfmt. Generated code is valid but not formatted");
            }
        }
    };
    // Add license file
    fs::write(destination_folder.join("LICENSE.txt"), ir.license_text)?;

    info!("Completed code generation");
    Ok(())
}
