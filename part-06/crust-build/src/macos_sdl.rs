use crate::{
    core::{context::Context, failable::Failable, failable_unit::FailableUnit, io, logs, remote_zips, script::Script, scripts},
    log_tag,
};
use std::path::PathBuf;

const SDL2_URL: &str = "https://www.libsdl.org/release/SDL2-2.0.14.zip";
const SDL2_DIR: &str = "SDL2";
const SDL2_FRAMEWORK_NAME: &str = "SDL2.framework";

const SDL2_IMAGE_URL: &str = "https://www.libsdl.org/projects/SDL_image/release/SDL2_image-2.0.5.zip";
const SDL2_IMAGE_DIR: &str = "SDL2_image";
const SDL2_IMAGE_FRAMEWORK_NAME: &str = "SDL2_image.framework";
const SDL2_IMAGE_CUSTOM_FRAMEWORK_DIR: &str = "SDL2_image_custom_framework";

pub fn setup(context: &Context) -> Failable<PathBuf> {
    let frameworks_dir = context.working_dir.join("Frameworks");

    io::create_dir(&frameworks_dir)?;
    setup_sdl2(context, &frameworks_dir)?;
    setup_sdl2_image(context, &frameworks_dir)?;

    Ok(frameworks_dir)
}

fn setup_sdl2(context: &Context, frameworks_dir: &PathBuf) -> FailableUnit {
    let output_dir = frameworks_dir.join(SDL2_FRAMEWORK_NAME);
    if output_dir.exists() {
        return Ok(());
    }

    remote_zips::fetch(SDL2_URL, SDL2_DIR, &context.working_dir)?;

    let xcode_project_dir = context.working_dir.join(SDL2_DIR).join("Xcode").join("SDL");
    logs::out(log_tag!(), "Compiling Xcode framework for SDL2, this may take a while ...");

    scripts::run(&Script::new(
		r#"xcodebuild archive -scheme Framework -destination "platform=macOS" -archivePath ./SDL2.xcarchive SKIP_INSTALL=NO BUILD_LIBRARY_FOR_DISTRIBUTION=YES"#
	).working_dir(&xcode_project_dir))?;

    io::copy(
        &xcode_project_dir.join("SDL2.xcarchive").join("Products").join("Library").join("Frameworks").join(SDL2_FRAMEWORK_NAME),
        &output_dir,
    )
}

fn setup_sdl2_image(context: &Context, frameworks_dir: &PathBuf) -> FailableUnit {
    let output_dir = frameworks_dir.join(SDL2_IMAGE_FRAMEWORK_NAME);

    if output_dir.exists() {
        return Ok(());
    }

    remote_zips::fetch(SDL2_IMAGE_URL, SDL2_IMAGE_DIR, &context.working_dir)?;

    let custom_framework_dir = context.working_dir.join(SDL2_IMAGE_CUSTOM_FRAMEWORK_DIR);

    io::delete(&custom_framework_dir)?;
    io::create_dir(&custom_framework_dir)?;
    io::create_symlink(&context.working_dir.join(SDL2_IMAGE_DIR), &PathBuf::from("Source"), &custom_framework_dir)?;
    io::create_symlink(frameworks_dir, &PathBuf::from("Frameworks"), &custom_framework_dir)?;
    io::write_string(SDL2_IMAGE_CUSTOM_FRAMEWORK_PROJECT_DEFINITION, &custom_framework_dir.join("project.yml"))?;

    scripts::run(&Script::new("xcodegen generate").working_dir(&custom_framework_dir))?;

    logs::out(log_tag!(), "Compiling custom Xcode framework for SDL2_image, this may take a while ...");

    scripts::run(&Script::new(
		r#"xcodebuild archive -scheme SDL2_image -destination "platform=macOS" -archivePath ./SDL2_image.xcarchive SKIP_INSTALL=NO BUILD_LIBRARY_FOR_DISTRIBUTION=YES"#
	).working_dir(&custom_framework_dir))?;

    logs::out(log_tag!(), "Copying SDL2_image.framework into Frameworks directory ...");

    io::copy(
        &custom_framework_dir
            .join("SDL2_image.xcarchive")
            .join("Products")
            .join("Library")
            .join("Frameworks")
            .join(SDL2_IMAGE_FRAMEWORK_NAME),
        &output_dir,
    )
}

const SDL2_IMAGE_CUSTOM_FRAMEWORK_PROJECT_DEFINITION: &str = r#"
name: SDL2_image
options:
    createIntermediateGroups: true
    deploymentTarget:
        macOS: "10.12"

targets:
    SDL2_image:
        type: framework
        platform: macOS
        info:
            path: Generated/Info.plist
            properties:
                CFBundleIdentifier: org.libsdl.SDL2-image
                CFBundleVersion: 2.0.5
                CFBundleShortVersionString: 2.0.5
        entitlements:
            path: Generated/app.entitlements
        sources:
            - Source/SDL_image.h
            - Source/IMG.c
            - Source/IMG_ImageIO.m
            - Source/IMG_bmp.c
            - Source/IMG_gif.c
            - Source/IMG_jpg.c
            - Source/IMG_lbm.c
            - Source/IMG_pcx.c
            - Source/IMG_png.c
            - Source/IMG_pnm.c
            - Source/IMG_svg.c
            - Source/IMG_tga.c
            - Source/IMG_tif.c
            - Source/IMG_webp.c
            - Source/IMG_xcf.c
            - Source/IMG_xpm.c
            - Source/IMG_xv.c
            - Source/IMG_xxx.c
        settings:
            DYLIB_COMPATIBILITY_VERSION: 3.0.0
            DYLIB_CURRENT_VERSION: 3.2.0
            CLANG_ENABLE_OBJC_ARC: NO
            GCC_PREPROCESSOR_DEFINITIONS:
                - "$(inherited)"
                - "LOAD_BMP"
                - "LOAD_JPG"
                - "LOAD_PNG"
            HEADER_SEARCH_PATHS:
                - $(SRCROOT)/../SDL2/include
            LIBRARY_SEARCH_PATHS:
                - $(inherited)
                - $(PROJECT_DIR)
                - $(PROJECT_DIR)/Frameworks
        dependencies:
            - framework: Frameworks/SDL2.framework
              embed: false
            - sdk: ApplicationServices.framework
            - sdk: Foundation.framework
"#;
