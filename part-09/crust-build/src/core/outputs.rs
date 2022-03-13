use crate::{
    core::{context::Context, failable_unit::FailableUnit, io, logs},
    log_tag,
};
use std::{path::PathBuf, vec::Vec};

pub fn output_dir(context: &Context) -> PathBuf {
    context.target_home_dir.join("out").join(context.variant.id())
}

pub fn clean(context: &Context) -> FailableUnit {
    io::delete(&output_dir(context))
}

pub fn collect(context: &Context, sources: Vec<PathBuf>) -> FailableUnit {
    io::create_dir(&output_dir(context))?;

    for source in sources {
        logs::out(log_tag!(), &format!("Collecting: {:?}", source));

        if source.is_dir() {
            io::copy(&source, &output_dir(context))?;
        } else {
            io::copy(&source, &output_dir(context).join(source.file_name().ok_or("Missing file name")?))?;
        }
    }

    Ok(())
}
