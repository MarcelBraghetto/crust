const DEBUG: &str = "debug";
const RELEASE: &str = "release";
const RUST_COMPILER_FLAG_DEBUG: &str = "";
const RUST_COMPILER_FLAG_RELEASE: &str = "--release";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Debug,
    Release,
}

impl Variant {
    pub fn resolve(arg: &str) -> Result<Variant, &str> {
        match &*arg.to_lowercase() {
            DEBUG => Ok(Variant::Debug),
            RELEASE => Ok(Variant::Release),
            _ => Err("Unknown variant"),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Variant::Debug => DEBUG,
            Variant::Release => RELEASE,
        }
    }

    pub fn rust_compiler_flag(&self) -> &str {
        match self {
            Variant::Debug => RUST_COMPILER_FLAG_DEBUG,
            Variant::Release => RUST_COMPILER_FLAG_RELEASE,
        }
    }
}
