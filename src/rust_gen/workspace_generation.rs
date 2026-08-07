use super::{
    Context, GenPkgSettings, Path, PathBuf, Target, Tera, execute_template, format_with_rustfmt,
    fs, generate_aurix_core_ir, generate_cortex_m_extra_files, generate_peripheral_module,
    get_tera_instance, info, ir, load_ir, read_license_file,
};
/// Collect the set of *other* peripheral module ids whose types are referenced
/// by the registers/clusters of a peripheral.
///
/// A peripheral references another one when it has a register or cluster that is
/// derived from (or otherwise shares the struct module of) a peripheral with a
/// different `module_id`. In workspace mode these references map to a path
/// dependency on the corresponding peripheral crate.
///
/// # Arguments
///
/// * `peri` - The peripheral module to analyze for external module references
/// * `own_module_id` - The module ID of the peripheral being analyzed (used to filter out self-references)
///
/// # Returns
///
/// A set of all unique external module IDs referenced by the peripheral's registers and clusters.
fn collect_referenced_modules(
    peri: &ir::PeripheralMod,
    own_module_id: &str,
) -> std::collections::BTreeSet<String> {
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

    let mut acc = std::collections::BTreeSet::new();
    for reg in peri.registers.values() {
        // first element of struct_module_path is the module_id of the peripheral that owns the struct
        // in other words the peripheral name
        if let Some(first) = reg.borrow().struct_module_path.first() {
            if first != own_module_id {
                acc.insert(first.clone());
            }
        }
    }
    for cluster in peri.clusters.values() {
        visit_cluster(&cluster.borrow(), own_module_id, &mut acc);
    }
    acc
}

/// Generate the per-peripheral crates for a Cargo workspace.
///
/// For each non-derived peripheral a crate is created under
/// `destination_folder/peripherals/<module_id>/` containing `src/<module_id>.rs`,
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
        let peri_dir = destination_folder.join("peripherals").join(&module_id);

        // Collect cross-peripheral references so they can be added as path
        // dependencies of this peripheral crate.
        let referenced = collect_referenced_modules(&peri.borrow(), &module_id);
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
pub fn generate_rust_workspace(
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
    } = *settings;

    info!("Start generating rust workspace");

    // Read license file if specified
    let custom_license_text = read_license_file(license_file.as_ref())?;
    let ir = load_ir(xml_path, svd_validation_level, custom_license_text.as_ref())?;

    // Precompile templates
    let tera = get_tera_instance()?;

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
