use crate::{
    core::{
        context::Context, failable_unit::FailableUnit, io, logs, manifests, remote_zips, script::Script, scripts,
        variant::Variant,
    },
    log_tag,
};
use std::{collections::HashMap, path::PathBuf};

const SDL2_SOURCE_URL: &str = "https://www.libsdl.org/release/SDL2-2.0.14.zip";
const SDL2_SOURCE_DIR: &str = "SDL";

const SDL2_IMAGE_SOURCE_URL: &str = "https://www.libsdl.org/projects/SDL_image/release/SDL2_image-2.0.5.zip";
const SDL2_IMAGE_SOURCE_DIR: &str = "SDL2_image";

const CRUST_SO_FILE_NAME: &str = "libcrustlib.so";

pub fn build(context: &Context) -> FailableUnit {
    context.print_summary();

    let ndk_dir = PathBuf::from(std::env::var("ANDROID_NDK_ROOT")?);
    logs::out(log_tag!(), &format!("Using Android NDK: {:?}", &ndk_dir));

    install_rust_dependencies()?;
    setup_sdl2(context, &ndk_dir)?;
    setup_assets(context)?;
    setup_cargo_manifest(context)?;
    compile_rust_code(context, &ndk_dir)?;
    link_jni_libs(context)?;

    Ok(())
}

fn install_rust_dependencies() -> FailableUnit {
    logs::out(log_tag!(), "Installing Android Rust targets ...");
    scripts::run(&Script::new(&format!(
        "rustup target add {} {} {} {}",
        Architecture::ARMv8A.rust_triple(),
        Architecture::ARMv7A.rust_triple(),
        Architecture::X86.rust_triple(),
        Architecture::X86_64.rust_triple(),
    )))
}

fn ndk_project_dir(context: &Context) -> PathBuf {
    context.working_dir.join("ndk")
}

fn compiled_libs_dir(context: &Context) -> PathBuf {
    ndk_project_dir(context).join("libs")
}

fn setup_sdl2(context: &Context, ndk_dir: &PathBuf) -> FailableUnit {
    let ndk_project_dir = ndk_project_dir(context);

    io::create_dir(&ndk_project_dir)?;

    remote_zips::fetch(SDL2_SOURCE_URL, SDL2_SOURCE_DIR, &ndk_project_dir)?;
    remote_zips::fetch(SDL2_IMAGE_SOURCE_URL, SDL2_IMAGE_SOURCE_DIR, &ndk_project_dir)?;

    let sdl_java_source_symlink = context.target_home_dir.join("app").join("src").join("main").join("java").join("org");
    io::create_symlink(
        &ndk_project_dir
            .join(SDL2_SOURCE_DIR)
            .join("android-project")
            .join("app")
            .join("src")
            .join("main")
            .join("java")
            .join("org"),
        &sdl_java_source_symlink,
        &context.target_home_dir,
    )?;

    io::write_string("include $(call all-subdir-makefiles)", &ndk_project_dir.join("Android.mk"))?;
    logs::out(log_tag!(), "Compiling SDL NDK libraries (this may take a while!) ...");
    io::delete(&compiled_libs_dir(context))?;

    scripts::run(
        &Script::new(&format!(
            "{:?} NDK_PROJECT_PATH={:?} APP_BUILD_SCRIPT={:?} APP_PLATFORM=android-21 APP_STL=c++_shared APP_ABI=all",
            &ndk_dir.join("ndk-build"),
            &ndk_project_dir,
            &ndk_project_dir.join("Android.mk"),
        ))
        .working_dir(&ndk_project_dir),
    )
}

fn setup_assets(context: &Context) -> FailableUnit {
    let app_assets_dir = context.target_home_dir.join("app").join("src").join("main").join("assets");
    let app_assets_symlink_dir = app_assets_dir.join("assets");

    io::create_dir(&app_assets_dir)?;
    io::create_symlink(&context.assets_dir, &app_assets_symlink_dir, &context.target_home_dir)?;

    Ok(())
}

fn setup_cargo_manifest(context: &Context) -> FailableUnit {
    logs::out(log_tag!(), "Creating Android Cargo.toml file ...");
    manifests::create(context, "cdylib")
}

fn compile_rust_code(context: &Context, ndk_dir: &PathBuf) -> FailableUnit {
    let is_windows = cfg!(target_os = "windows");

    let ndk_toolchain_dir = ndk_dir
        .join("toolchains")
        .join("llvm")
        .join("prebuilt")
        .join(if is_windows { "windows-x86_64" } else { "darwin-x86_64" })
        .join("bin");

    logs::out(log_tag!(), &format!("Using NDK toolchain at: {:?}", &ndk_toolchain_dir));

    for architecture in &vec![
        Architecture::ARMv8A,
        Architecture::ARMv7A,
        Architecture::X86,
        Architecture::X86_64,
    ] {
        let rust_triple = architecture.rust_triple();
        let ndk_triple = architecture.ndk_triple();
        let cargo_rust_triple = rust_triple.to_uppercase().replace("-", "_");

        logs::out(log_tag!(), &format!("Compiling architecture: {:?}", &rust_triple));

        let mut environment = HashMap::new();

        environment.insert(
            format!("CARGO_TARGET_{}_AR", &cargo_rust_triple),
            ndk_toolchain_dir
                .join(if is_windows {
                    format!(r"{}-ar.exe", ndk_triple)
                } else {
                    format!(r"{}-ar", ndk_triple)
                })
                .display()
                .to_string(),
        );

        environment.insert(
            format!("CARGO_TARGET_{}_LINKER", &cargo_rust_triple),
            ndk_toolchain_dir
                .join(if is_windows {
                    format!(r"{}30-clang.cmd", ndk_triple)
                } else {
                    format!(r"{}30-clang", ndk_triple)
                })
                .display()
                .to_string(),
        );

        environment.insert(
            format!("CARGO_TARGET_{}_RUSTFLAGS", &cargo_rust_triple),
            format!(
                "-Clink-arg=-L{} -lc++_shared -lhidapi -lSDL2 -lSDL2_image",
                &compiled_libs_dir(context).join(architecture.jni_name()).display().to_string()
            ),
        );

        scripts::run(
            &Script::new(&format!(
                "cargo rustc {} --target-dir {:?} --lib --target {}",
                context.variant.rust_compiler_flag(),
                context.rust_build_dir,
                rust_triple,
            ))
            .environment(&environment)
            .working_dir(&context.working_dir),
        )?;

        let compiled_crust_so_path = context.rust_build_dir.join(rust_triple).join(context.variant.id()).join(CRUST_SO_FILE_NAME);

        if context.variant == Variant::Release {
            logs::out(log_tag!(), &format!("Stripping .so library: {:?}", &compiled_crust_so_path));
            let strip_triple = architecture.strip_triple();
            let strip_tool = ndk_toolchain_dir.join(if is_windows {
                format!(r"{}-strip.exe", strip_triple)
            } else {
                format!(r"{}-strip", strip_triple)
            });

            scripts::run(
                &Script::new(&format!("{:?} {:?}", &strip_tool, &compiled_crust_so_path)).working_dir(&context.working_dir),
            )?;
        }

        io::copy(&compiled_crust_so_path, &compiled_libs_dir(context).join(architecture.jni_name()).join(CRUST_SO_FILE_NAME))?;
    }

    Ok(())
}

fn link_jni_libs(context: &Context) -> FailableUnit {
    let app_jni_libs_dir = context.target_home_dir.join("app").join("src").join("main").join("jniLibs");

    logs::out(log_tag!(), "Linking 'libs' into Android app 'jniLibs' ...");
    io::create_symlink(&compiled_libs_dir(context), &app_jni_libs_dir, &context.target_home_dir)?;

    Ok(())
}

enum Architecture {
    ARMv8A,
    ARMv7A,
    X86,
    X86_64,
}

impl Architecture {
    fn jni_name(&self) -> String {
        String::from(match self {
            Architecture::ARMv8A => "arm64-v8a",
            Architecture::ARMv7A => "armeabi-v7a",
            Architecture::X86 => "x86",
            Architecture::X86_64 => "x86_64",
        })
    }

    fn ndk_triple(&self) -> String {
        String::from(match self {
            Architecture::ARMv8A => "aarch64-linux-android",
            Architecture::ARMv7A => "armv7a-linux-androideabi",
            Architecture::X86 => "i686-linux-android",
            Architecture::X86_64 => "x86_64-linux-android",
        })
    }

    fn rust_triple(&self) -> String {
        String::from(match self {
            Architecture::ARMv8A => "aarch64-linux-android",
            Architecture::ARMv7A => "armv7-linux-androideabi",
            Architecture::X86 => "i686-linux-android",
            Architecture::X86_64 => "x86_64-linux-android",
        })
    }

    fn strip_triple(&self) -> String {
        String::from(match self {
            Architecture::ARMv8A => "aarch64-linux-android",
            Architecture::ARMv7A => "arm-linux-androideabi",
            Architecture::X86 => "i686-linux-android",
            Architecture::X86_64 => "x86_64-linux-android",
        })
    }
}
