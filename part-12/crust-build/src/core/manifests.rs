use crate::{
    core::{
        failable_unit::FailableUnit,
        {context::Context, io, logs},
    },
    log_tag,
};

pub fn create(context: &Context, crate_type: &str) -> FailableUnit {
    logs::out(log_tag!(), "Creating custom Cargo.toml manifest ...");

    let main_source_dir = context.source_dir.join("src");
    let manifest_content = io::read_string(&context.source_dir.join("Cargo.toml"))?;
    let mut manifest = manifest_content.parse::<toml_edit::Document>()?;

    let lib_src = format!("{:?}", &main_source_dir.join("lib.rs"));
    manifest["lib"]["path"] = toml_edit::value(lib_src);

    let crate_types = manifest["lib"]["crate-type"].as_array_mut().ok_or("Field 'lib/crate-type' not found in manifest!")?;

    for i in 0..crate_types.iter().count() {
        crate_types.remove(i);
    }

    crate_types.push(crate_type).map_err(|_| "Failed to set manifest crate-type")?;

    let bin_src = format!("{:?}", &main_source_dir.join("bin.rs"));
    manifest["bin"]
        .as_array_of_tables_mut()
        .ok_or("Missing 'bin' manifest entry")?
        .get_mut(0)
        .ok_or("Missing 'bin' manifest element 0")?["path"] = toml_edit::value(bin_src);

    io::write_string(&manifest.to_string(), &context.working_dir.join("Cargo.toml"))
}
