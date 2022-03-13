use crate::{
    core::{context::Context, failable_unit::FailableUnit, io, logs, script::Script, scripts},
    log_tag, macos_sdl,
};
use std::path::PathBuf;

const ARCHITECTURE_X86_64: &str = "x86_64-apple-darwin";
const ARCHITECTURE_ARM64: &str = "aarch64-apple-darwin";

pub fn build(context: &Context) -> FailableUnit {
    context.print_summary();

    install_rust_dependencies()?;

    let frameworks_dir = macos_sdl::setup(context)?;
    link_frameworks(context, &frameworks_dir)?;
    compile(context)?;
    create_output(context)?;

    Ok(())
}

fn install_rust_dependencies() -> FailableUnit {
    scripts::run(&Script::new(&format!("rustup target add {} {}", ARCHITECTURE_X86_64, ARCHITECTURE_ARM64)))
}

fn link_frameworks(context: &Context, frameworks_dir: &PathBuf) -> FailableUnit {
    io::create_symlink(frameworks_dir, &context.target_home_dir.join("crust").join("Frameworks"), &context.working_dir)
}

fn compile(context: &Context) -> FailableUnit {
    for architecture in &vec![ARCHITECTURE_X86_64, ARCHITECTURE_ARM64] {
        logs::out(log_tag!(), &format!("Compiling architecture: {} ...", &architecture));

        scripts::run(&Script::new(&format!(
            "cargo rustc {} --manifest-path {:?} --target {} --bin crust --target-dir {:?} -- -L framework={:?}",
            context.variant.rust_compiler_flag(),
            context.source_dir.join("Cargo.toml"),
            &architecture,
            context.rust_build_dir,
            context.working_dir.join("Frameworks"),
        )))?;
    }

    Ok(())
}

fn create_output(context: &Context) -> FailableUnit {
    let output_binary_dir = context.target_home_dir.join("crust").join("crust");
    let x86_64_binary = context.rust_build_dir.join(ARCHITECTURE_X86_64).join(context.variant.id()).join("crust");
    let arm64_binary = context.rust_build_dir.join(ARCHITECTURE_ARM64).join(context.variant.id()).join("crust");

    scripts::run(
        &Script::new(&format!("lipo -create -output crust {:?} {:?}", &x86_64_binary, &arm64_binary))
            .working_dir(&output_binary_dir),
    )?;

    scripts::run(&Script::new("install_name_tool -add_rpath @loader_path/../Frameworks crust").working_dir(&output_binary_dir))
}
