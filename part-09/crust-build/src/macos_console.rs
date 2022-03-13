use crate::{
    core::{context::Context, failable_unit::FailableUnit, io, logs, outputs, script::Script, scripts, variant::Variant},
    log_tag, macos_sdl,
};
use std::path::PathBuf;

pub fn build(context: &Context) -> FailableUnit {
    context.print_summary();
    let frameworks_dir = macos_sdl::setup(context)?;
    compile(context, &frameworks_dir)?;
    create_output(context, &frameworks_dir)
}

fn compile(context: &Context, frameworks_dir: &PathBuf) -> FailableUnit {
    logs::out(log_tag!(), "Compiling application ...");

    scripts::run(&Script::new(&format!(
        r#"cargo rustc {} --manifest-path {:?} --bin crust --target-dir {:?} -- -L framework={:?}"#,
        context.variant.rust_compiler_flag(),
        context.source_dir.join("Cargo.toml"),
        context.rust_build_dir,
        frameworks_dir,
    )))?;

    logs::out(log_tag!(), "Compile completed successfully!");

    Ok(())
}

fn create_output(context: &Context, frameworks_dir: &PathBuf) -> FailableUnit {
    let output_dir = outputs::output_dir(context);

    logs::out(log_tag!(), "Creating product ...");

    outputs::clean(context)?;
    outputs::collect(context, vec![context.rust_build_dir.join(context.variant.id()).join("crust")])?;

    match context.variant {
        Variant::Debug => {
            logs::out(log_tag!(), "Debug build - symlinking assets ...");
            io::create_symlink(&context.assets_dir, &PathBuf::from("assets"), &output_dir)?;

            logs::out(log_tag!(), "Debug build - symlinking frameworks ...");
            io::create_symlink(&frameworks_dir, &PathBuf::from("Frameworks"), &output_dir)?;
        }

        Variant::Release => {
            logs::out(log_tag!(), "Release build - copying assets ...");
            outputs::collect(context, vec![context.assets_dir.clone()])?;

            logs::out(log_tag!(), "Release build - copying frameworks ...");
            outputs::collect(context, vec![frameworks_dir.clone()])?;
        }
    }

    scripts::run(&Script::new("install_name_tool -add_rpath @loader_path/Frameworks crust").working_dir(&output_dir))
}
