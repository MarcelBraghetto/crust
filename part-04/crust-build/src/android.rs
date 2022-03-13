use crate::core::{context::Context, failable_unit::FailableUnit};

pub fn build(context: &Context) -> FailableUnit {
    context.print_summary();
    Ok(())
}
