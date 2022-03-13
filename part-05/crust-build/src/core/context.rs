use crate::{
    core::{logs, target::Target, variant::Variant},
    log_tag,
};
use std::path::PathBuf;

pub struct Context {
    pub assets_dir: PathBuf,
    pub rust_build_dir: PathBuf,
    pub source_dir: PathBuf,
    pub target_home_dir: PathBuf,
    pub variant: Variant,
    pub working_dir: PathBuf,
}

impl Context {
    pub fn new(root_dir: PathBuf, target: Target, variant: Variant) -> Self {
        let target_home_dir = root_dir.join(target.id());
        let working_dir = target_home_dir.join(".rust-build");
        let rust_build_dir = working_dir.join("rust");
        let source_dir = root_dir.join("crust-main");
        let assets_dir = source_dir.join("assets");

        Context {
            assets_dir: assets_dir,
            rust_build_dir: rust_build_dir,
            source_dir: source_dir,
            target_home_dir: target_home_dir,
            variant: variant,
            working_dir: working_dir,
        }
    }

    pub fn print_summary(&self) {
        logs::out(log_tag!(), "---------------------------------------------");
        logs::out(log_tag!(), &format!("Assets dir:          {:?}", self.assets_dir));
        logs::out(log_tag!(), &format!("Working dir:         {:?}", self.working_dir));
        logs::out(log_tag!(), &format!("Rust build dir:      {:?}", self.rust_build_dir));
        logs::out(log_tag!(), &format!("Variant:             {:?}", self.variant));
        logs::out(log_tag!(), &format!("Target home dir:     {:?}", self.target_home_dir));
        logs::out(log_tag!(), &format!("Main source dir:     {:?}", self.source_dir));
        logs::out(log_tag!(), "---------------------------------------------");
    }
}
