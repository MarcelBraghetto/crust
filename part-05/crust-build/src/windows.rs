use crate::{
    core::{
        context::Context, failable::Failable, failable_unit::FailableUnit, logs, outputs, remote_zips, script::Script, scripts,
    },
    log_tag,
};
use std::path::PathBuf;

const SDL2_URL: &str = "https://www.libsdl.org/release/SDL2-devel-2.0.14-VC.zip";
const SDL2_DIR: &str = "sdl2";

const SDL2_IMAGE_URL: &str = "https://www.libsdl.org/projects/SDL_image/release/SDL2_image-devel-2.0.5-VC.zip";
const SDL2_IMAGE_DIR: &str = "sdl2-image";

pub fn build(context: &Context) -> FailableUnit {
    context.print_summary();
    let sdl2_libs_dir = setup_sdl2(context)?;
    let sdl2_image_libs_dir = setup_sdl2_image(context)?;
    compile(context, &sdl2_libs_dir, &sdl2_image_libs_dir)?;
    create_output(context, &sdl2_libs_dir, &sdl2_image_libs_dir)
}

fn setup_sdl2(context: &Context) -> Failable<PathBuf> {
    remote_zips::fetch(SDL2_URL, SDL2_DIR, &context.working_dir)?;
    Ok(context.working_dir.join(SDL2_DIR).join("lib").join("x64"))
}

fn setup_sdl2_image(context: &Context) -> Failable<PathBuf> {
    remote_zips::fetch(SDL2_IMAGE_URL, SDL2_IMAGE_DIR, &context.working_dir)?;
    Ok(context.working_dir.join(SDL2_IMAGE_DIR).join("lib").join("x64"))
}

fn compile(context: &Context, sdl2_libs_dir: &PathBuf, sdl2_image_libs_dir: &PathBuf) -> FailableUnit {
    logs::out(log_tag!(), "Compiling application ...");

    // When we compile our Rust code we will add extra linker search paths using the `-L` flag, so our build
    // can locate the appropriate SDL `.lib` files to link against. Note that the `.lib` file doesn't contain
    // the implementation - that is what the `.dll` files do and we'll collect them later in the build process.
    // If you add more external libraries you need to add a search path to the location of their .lib files too.
    scripts::run(&Script::new(&format!(
        r#"cargo rustc {} --manifest-path {:?} --bin crust --target-dir {:?} -- -L {:?} -L {:?}"#,
        context.variant.rust_compiler_flag(),
        context.source_dir.join("Cargo.toml"),
        context.rust_build_dir,
        sdl2_libs_dir,
        sdl2_image_libs_dir,
    )))?;

    logs::out(log_tag!(), "Compile completed successfully!");

    Ok(())
}

fn create_output(context: &Context, sdl2_libs_dir: &PathBuf, sdl2_image_libs_dir: &PathBuf) -> FailableUnit {
    logs::out(log_tag!(), "Creating product ...");
    outputs::clean(context)?;
    outputs::collect(
        context,
        vec![
            context.rust_build_dir.join(context.variant.id()).join("crust.exe"),
            sdl2_libs_dir.join("SDL2.dll"),
            sdl2_image_libs_dir.join("SDL2_image.dll"),
            sdl2_image_libs_dir.join("libpng16-16.dll"),
            sdl2_image_libs_dir.join("zlib1.dll"),
        ],
    )
}
