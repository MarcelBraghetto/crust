use crate::{
    core::{context::Context, failable_unit::FailableUnit, logs, outputs, script::Script, scripts},
    log_tag,
};

pub fn build(context: &Context) -> FailableUnit {
    context.print_summary();
    compile(context)?;
    create_output(context)?;
    Ok(())
}

fn compile(context: &Context) -> FailableUnit {
    logs::out(log_tag!(), "Compiling application ...");

    scripts::run(&Script::new(&format!(
        r#"cargo rustc {} --manifest-path {:?} --bin crust --target-dir {:?}"#,
        context.variant.rust_compiler_flag(),
        context.source_dir.join("Cargo.toml"),
        context.rust_build_dir,
    )))?;

    logs::out(log_tag!(), "Compile completed successfully!");

    Ok(())
}

fn create_output(context: &Context) -> FailableUnit {
    logs::out(log_tag!(), "Creating product ...");
    outputs::clean(context)?;
    outputs::collect(context, vec![context.rust_build_dir.join(context.variant.id()).join("crust")])
}
