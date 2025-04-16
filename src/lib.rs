#![doc = include_str!("../README.md")]

mod rust_gen;
mod svd_util;
use crate::rust_gen::{generate_rust_package, GenPkgSettings};
use clap::{Parser, ValueEnum};
use env_logger::Env;
use log::{error, info, warn};
use serde::Serialize;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum SvdValidationLevel {
    Disabled,
    Weak,
    Strict,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize)]
pub enum Target {
    /// Only generic access to registers. No support for interrupt vector and NVIC priority bits.
    Generic,
    /// Generic access to register + atomic read modify store of register values. Dependant on tricore crate.
    Aurix,
    /// Support for interrupt vector and NVIC priority bits. Compatible with existing cortex-m-rt crate
    CortexM,
}

/// Generate peripheral access crate from SVD file
#[derive(Parser, Debug)]
#[command(author, version=env!("CARGO_PKG_VERSION"), about="Tool to generate peripheral access crate from SVD file", long_about = None)]
pub struct Args {
    /// Disable formatting of generated code using rustfmt mainly for debugging
    #[arg(long,value_parser=clap::value_parser!(bool),default_value_t=false)]
    pub disable_rust_fmt: bool,
    /// Register description file
    #[arg(value_parser=clap::value_parser!(PathBuf))]
    pub register_description_file_name: PathBuf,
    /// Destination folder of package
    #[arg(value_parser=clap::value_parser!(PathBuf))]
    pub destination_folder: PathBuf,
    //SVD validation level
    #[arg(long,value_enum,default_value_t=SvdValidationLevel::Weak)]
    pub svd_validation_level: SvdValidationLevel,
    /// Architecture target of the PAC.
    #[arg(long,value_enum,default_value_t=Target::Generic)]
    pub target: Target,
    /// Enable the generation of a PAC with the tracing interface.
    #[arg(long,value_parser=clap::value_parser!(bool),default_value_t=false)]
    pub tracing: bool,
    /// Define package name in toml. Default is name stored in register description file
    #[arg(long,value_parser=clap::value_parser!(String),default_value=None)]
    pub package_name: Option<String>,
    /// Specify a license file whose content is used instead of one defined in SVD.
    #[arg(long,value_parser=clap::value_parser!(PathBuf),default_value=None)]
    pub license_file: Option<PathBuf>,
}

/// Main function that parses command line parameters after parsing it invoking [`main`]
///
/// # Arguments
///
/// * `args` - List of command line arguments. The first argument should be the path of executable, but it is ignored by this function.
///
/// # Examples
///
/// ```ignore
/// let args = ["", "./test_svd/simple.xml", "./generated_code"];
/// main_parse_arguments(args);
/// ```
pub fn main_parse_arguments<I, T>(args: I)
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    self::main(Args::parse_from(args));
}

/// Convert SVD file to PAC
pub fn main(args: Args) {
    // Use
    let env = Env::default()
        .filter_or("SVD2PAC_LOG_LEVEL", "info")
        .write_style_or("SVD2PAC_LOG_STYLE", "always");

    // During test cases the logger is already initialized.
    // Just show a warn
    if let Err(error) = env_logger::try_init_from_env(env) {
        warn!("{}", error);
    }

    info!(
        "Reading register description file {}",
        args.register_description_file_name.to_str().unwrap()
    );
    let destination_folder = args.destination_folder;

    if !destination_folder.exists() {
        info!("Create folder {}", &destination_folder.to_str().unwrap());
        if let Err(err) = fs::create_dir_all(&destination_folder) {
            error!("Failed to create destination folder: {}", err);
            exit(-1);
        };
    }

    if let Err(err) = generate_rust_package(
        &args.register_description_file_name,
        &destination_folder,
        GenPkgSettings {
            run_rustfmt: !args.disable_rust_fmt,
            svd_validation_level: args.svd_validation_level,
            target: args.target,
            tracing: args.tracing,
            package_name: args.package_name,
            license_file: args.license_file,
            svd2pac_version: VERSION.to_owned(),
        },
    ) {
        error!("Failed to generate code with err {}", err);
        exit(-1);
    }
}
