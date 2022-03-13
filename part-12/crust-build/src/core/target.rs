use crate::core::failable::Failable;

const ANDROID: &str = "android";
const EMSCRIPTEN: &str = "emscripten";
const IOS: &str = "ios";
const MACOS_CONSOLE: &str = "macos-console";
const MACOS_DESKTOP: &str = "macos-desktop";
const WINDOWS: &str = "windows";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Android,
    Emscripten,
    Ios,
    MacOSConsole,
    MacOSDesktop,
    Windows,
}

impl Target {
    pub fn resolve(id: &str) -> Failable<Target> {
        match &*id.to_lowercase() {
            ANDROID => Ok(Target::Android),
            EMSCRIPTEN => Ok(Target::Emscripten),
            IOS => Ok(Target::Ios),
            MACOS_CONSOLE => Ok(Target::MacOSConsole),
            MACOS_DESKTOP => Ok(Target::MacOSDesktop),
            WINDOWS => Ok(Target::Windows),
            _ => Err("Unknown target id".into()),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Target::Android => ANDROID,
            Target::Emscripten => EMSCRIPTEN,
            Target::Ios => IOS,
            Target::MacOSConsole => MACOS_CONSOLE,
            Target::MacOSDesktop => MACOS_DESKTOP,
            Target::Windows => WINDOWS,
        }
    }
}
