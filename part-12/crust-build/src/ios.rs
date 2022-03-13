use crate::{
    core::{
        context::Context, failable::Failable, failable_unit::FailableUnit, io, logs, manifests, remote_zips, script::Script,
        scripts,
    },
    log_tag,
};
use std::path::PathBuf;

const ARCHITECTURE_ARM64_IPHONE: &str = "aarch64-apple-ios";
const ARCHITECTURE_ARM64_SIMULATOR: &str = "aarch64-apple-ios-sim";
const ARCHITECTURE_X86_64_SIMULATOR: &str = "x86_64-apple-ios";

const SDL2_URL: &str = "https://www.libsdl.org/release/SDL2-2.0.14.zip";
const SDL2_DIR: &str = "SDL";
const SDL2_FRAMEWORK_NAME: &str = "SDL2.xcframework";

const SDL2_IMAGE_URL: &str = "https://www.libsdl.org/projects/SDL_image/release/SDL2_image-2.0.5.zip";
const SDL2_IMAGE_DIR: &str = "SDL2_image";
const SDL2_IMAGE_FRAMEWORK_NAME: &str = "SDL2_image.xcframework";

pub fn build(context: &Context) -> FailableUnit {
    context.print_summary();

    install_rust_dependencies()?;

    let frameworks_dir = setup_frameworks_dir(context)?;
    setup_sdl2(context, &frameworks_dir)?;
    setup_sdl2_image(context, &frameworks_dir)?;
    manifests::create(context, "staticlib")?;
    compile(context)?;
    create_output(context, &frameworks_dir)?;

    Ok(())
}

fn install_rust_dependencies() -> FailableUnit {
    scripts::run(&Script::new(&format!(
        "rustup target add {} {} {}",
        ARCHITECTURE_ARM64_IPHONE, ARCHITECTURE_ARM64_SIMULATOR, ARCHITECTURE_X86_64_SIMULATOR
    )))
}

fn setup_frameworks_dir(context: &Context) -> Failable<PathBuf> {
    let frameworks_dir = context.working_dir.join("Frameworks");
    io::create_dir(&frameworks_dir)?;
    io::create_symlink(&frameworks_dir, &context.target_home_dir.join("crust").join("Frameworks"), &context.working_dir)?;

    Ok(frameworks_dir)
}

fn setup_sdl2(context: &Context, frameworks_dir: &PathBuf) -> FailableUnit {
    let xcframework_path = frameworks_dir.join(SDL2_FRAMEWORK_NAME);

    if xcframework_path.exists() {
        return Ok(());
    }

    // The source code for SDL2 needs to be available for us to build the framework.
    remote_zips::fetch(SDL2_URL, SDL2_DIR, &context.working_dir)?;

    // This is the directory where the SDL2 Xcode project can be found which when compiled produces the static libraries we need.
    let xcode_project_dir = context.working_dir.join(SDL2_DIR).join("Xcode").join("SDL");

    // We need to build the static library for both simulators (ARM64 + x86_64). Note also that we are deliberately building the Release variant only.
    logs::out(log_tag!(), "Compiling SDL2 simulator release static library ...");
    scripts::run(&Script::new(
        r#"xcodebuild -project "SDL.xcodeproj" -scheme "Static Library-iOS" -configuration Release -sdk iphonesimulator -derivedDataPath "build_iphonesimulator" -arch arm64 -arch x86_64 only_active_arch=no"#
    ).working_dir(&xcode_project_dir))?;

    // We now need to build the static library for the phone target (ARM64).
    logs::out(log_tag!(), "Compiling SDL2 phone release static library ...");
    scripts::run(&Script::new(
        r#"xcodebuild -project "SDL.xcodeproj" -scheme "Static Library-iOS" -configuration Release -sdk iphoneos -derivedDataPath "build_iphoneos" -arch arm64 only_active_arch=no"#
    ).working_dir(&xcode_project_dir))?;

    // Once both the simulators + phone static libraries have been built, we need to merge them together into an XCFramework which will be embedded in the iOS project.
    let headers_path = context.working_dir.join(SDL2_DIR).join("include");
    logs::out(log_tag!(), "Creating SDL2 XCFramework ...");
    scripts::run(&Script::new(&format!(
        r#"xcodebuild -create-xcframework -library build_iphonesimulator/Build/Products/Release-iphonesimulator/libSDL2.a -headers {:?} -library build_iphoneos/Build/Products/Release-iphoneos/libSDL2.a -headers {:?} -output {:?}"#,
        &headers_path,
        &headers_path,
        &xcframework_path
    )).working_dir(&xcode_project_dir))?;

    Ok(())
}

fn setup_sdl2_image(context: &Context, frameworks_dir: &PathBuf) -> FailableUnit {
    let xcframework_path = frameworks_dir.join(SDL2_IMAGE_FRAMEWORK_NAME);

    // No need to build if its output already exist.
    if xcframework_path.exists() {
        return Ok(());
    }

    // The source code for SDL2 image needs to be available for us to build the framework.
    remote_zips::fetch(SDL2_IMAGE_URL, SDL2_IMAGE_DIR, &context.working_dir)?;

    // // This is the directory where the SDL2 Image Xcode project can be found which when compiled produces the static libraries we need.
    let xcode_project_dir = context.working_dir.join(SDL2_IMAGE_DIR).join("Xcode-iOS");

    // We need to build the static library for both simulators (ARM64 + x86_64). Note also that we are deliberately building the Release variant only.
    logs::out(log_tag!(), "Compiling SDL2 Image simulator release static library ...");
    scripts::run(&Script::new(
        r#"xcodebuild -project "SDL_image.xcodeproj" -scheme "libSDL_image-iOS" -configuration Release -sdk iphonesimulator -derivedDataPath "build_iphonesimulator" -arch arm64 -arch x86_64 only_active_arch=no"#
    ).working_dir(&xcode_project_dir))?;

    // We now need to build the static library for the phone target (ARM64).
    logs::out(log_tag!(), "Compiling SDL2 Image phone release static library ...");
    scripts::run(&Script::new(
        r#"xcodebuild -project "SDL_image.xcodeproj" -scheme "libSDL_image-iOS" -configuration Release -sdk iphoneos -derivedDataPath "build_iphoneos" -arch arm64 only_active_arch=no"#
    ).working_dir(&xcode_project_dir))?;

    // Once both the simulators + phone static libraries have been built, we need to merge them together into an XCFramework which will be embedded in the iOS project.
    // Note: We do not need to include the headers in the framework as we won't be accessing them in our iOS project directly.
    logs::out(log_tag!(), "Creating SDL2 Image XCFramework ...");
    scripts::run(&Script::new(&format!(
        r#"xcodebuild -create-xcframework -library build_iphonesimulator/Build/Products/Release-iphonesimulator/libSDL2_image.a -library build_iphoneos/Build/Products/Release-iphoneos/libSDL2_image.a -output {:?}"#,
    &xcframework_path)).working_dir(&xcode_project_dir))?;

    Ok(())
}

fn compile(context: &Context) -> FailableUnit {
    for architecture in &vec![
        ARCHITECTURE_ARM64_IPHONE,
        ARCHITECTURE_ARM64_SIMULATOR,
        ARCHITECTURE_X86_64_SIMULATOR,
    ] {
        logs::out(log_tag!(), &format!("Compiling crust for architecture: {}", &architecture));
        scripts::run(
            &Script::new(&format!(
                "cargo rustc {} --target-dir {:?} --lib --target {}",
                context.variant.rust_compiler_flag(),
                context.rust_build_dir,
                architecture,
            ))
            .working_dir(&context.working_dir),
        )?;
    }

    Ok(())
}

fn create_output(context: &Context, frameworks_dir: &PathBuf) -> FailableUnit {
    let variant_dir = context.variant.id();

    // Now that each of the architectures has been compiled, we need to generate an XCFramework which merges them all together for the iOS project.
    // When generating an XCFramework we must actually join any compiled architectures that belong to the same family together first - in our case
    // we have compiled 2 iOS simulator architectures (ARM64 + x86_64). If we were to try and make an XCFramework where each of the simulator
    // architectures were bundled separately, the XCFramework command would fail so we must first join them into 1 static library. To join compiled
    // architectures together we use the 'lipo' tool.
    logs::out(log_tag!(), "Joining iOS simulator architectures together ...");
    let simulator_static_library_dir = context
        .rust_build_dir
        .join(&format!("{}-{}", ARCHITECTURE_ARM64_SIMULATOR, ARCHITECTURE_X86_64_SIMULATOR))
        .join(variant_dir);

    // Make sure the path to our merged static library exists.
    io::create_dir(&simulator_static_library_dir)?;

    let simulator_static_library_path = simulator_static_library_dir.join("libcrustlib.a");

    scripts::run(
        &Script::new(&format!(
            "lipo -create -output {:?} {:?} {:?}",
            &simulator_static_library_path,
            &context.rust_build_dir.join(ARCHITECTURE_ARM64_SIMULATOR).join(variant_dir).join("libcrustlib.a"),
            &context.rust_build_dir.join(ARCHITECTURE_X86_64_SIMULATOR).join(variant_dir).join("libcrustlib.a"),
        ))
        .working_dir(&context.working_dir),
    )?;

    // Now that we have a static library for the iPhone architecture and a static library representing both iOS simulator architectures (via the lipo tool),
    // we can build the XCFramework which wraps it all up and can then be used as a framework dependency in the iOS project.
    let xcframework_path = frameworks_dir.join("crust.xcframework");
    let iphone_static_library_path =
        context.rust_build_dir.join(ARCHITECTURE_ARM64_IPHONE).join(variant_dir).join("libcrustlib.a");

    // Start off by deleting the existing framework if it exists - we should do this so building a debug/release variant always produces the appropriate XCFramework.
    io::delete(&xcframework_path)?;

    // Now run the appropriate Xcode command to create the framework.
    logs::out(log_tag!(), "Creating XCFramework for 'crust' ...");
    scripts::run(
        &Script::new(&format!(
            "xcodebuild -create-xcframework -library {:?} -library {:?} -output {:?}",
            &iphone_static_library_path, &simulator_static_library_path, &xcframework_path,
        ))
        .working_dir(&context.working_dir),
    )?;

    // The Frameworks directory now contains 'crust.xcframework' ready to be used in the iOS Xcode project.
    Ok(())
}
