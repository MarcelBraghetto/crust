pub mod core;
pub mod log_tag;

mod android;
mod emscripten;
mod ios;
mod macos_console;
mod macos_desktop;
mod macos_sdl;
mod windows;

use crate::core::{context::Context, failable_unit::FailableUnit, logs, target::Target, variant::Variant};
use clap::{App, AppSettings, Arg};
use std::path::PathBuf;

#[cfg(target_os = "windows")]
fn get_supported_platform_ids() -> Vec<&'static str> {
    vec![Target::Android.id(), Target::Emscripten.id(), Target::Windows.id()]
}

#[cfg(target_os = "macos")]
fn get_supported_platform_ids() -> Vec<&'static str> {
    vec![
        Target::Android.id(),
        Target::Emscripten.id(),
        Target::Ios.id(),
        Target::MacOSConsole.id(),
        Target::MacOSDesktop.id(),
    ]
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    let cli = App::new("crust-build")
        .version("1.0.0")
        .author("Marcel Braghetto")
        .about("CLI for building 'CRUST' targets.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("target")
                .long("target")
                .takes_value(true)
                .required(true)
                .possible_values(&get_supported_platform_ids())
                .case_insensitive(true)
                .help("Target:"),
        )
        .arg(
            Arg::with_name("variant")
                .long("variant")
                .takes_value(true)
                .possible_values(&[Variant::Debug.id(), Variant::Release.id()])
                .case_insensitive(true)
                .default_value(Variant::Debug.id())
                .help("Variant:"),
        )
        .get_matches();

    std::process::exit(match build(&cli) {
        Ok(_) => 0,
        Err(err) => {
            logs::out(log_tag!(), &format!("Fatal error: {:?}", err));
            1
        }
    });
}

fn build(cli: &clap::ArgMatches) -> FailableUnit {
    let current_dir = match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(manifest_path) => PathBuf::from(manifest_path),
        _ => {
            panic!("Run crust-build via 'cargo run'!");
        }
    };

    let target = Target::resolve(cli.value_of("target").ok_or("Target arg not found.")?)?;
    let variant = Variant::resolve(cli.value_of("variant").ok_or("Variant arg not found.")?)?;
    let context = Context::new(current_dir.parent().ok_or("Missing parent dir")?.to_path_buf(), target, variant);

    match target {
        Target::Android => android::build(&context),
        Target::Emscripten => emscripten::build(&context),
        Target::Ios => ios::build(&context),
        Target::MacOSConsole => macos_console::build(&context),
        Target::MacOSDesktop => macos_desktop::build(&context),
        Target::Windows => windows::build(&context),
    }
}
