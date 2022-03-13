use crate::{
    core::{context::Context, failable_unit::FailableUnit, io, logs, outputs, remote_zips, script::Script, scripts},
    log_tag,
};
use std::{collections::HashMap, path::PathBuf};

const EMSCRIPTEN_VERSION: &str = "2.0.32";
const EMSCRIPTEN_URL: &str = "https://github.com/emscripten-core/emsdk/archive/refs/tags/2.0.32.zip";

pub fn build(context: &Context) -> FailableUnit {
    context.print_summary();
    install_rust_dependencies()?;
    configure_emscripten_sdk(context)?;
    compile_application(context)?;
    create_output(context)?;
    open_in_browser(context)?;

    Ok(())
}

fn install_rust_dependencies() -> FailableUnit {
    logs::out(log_tag!(), "Installing Rust dependencies ...");

    scripts::run(&Script::new("rustup target add wasm32-unknown-emscripten"))?;
    scripts::run(&Script::new("cargo install --version 1.12.0 https"))
}

fn sdk_dir_name() -> String {
    format!("emscripten-sdk-{}", EMSCRIPTEN_VERSION)
}

fn sdk_dir(context: &Context) -> PathBuf {
    context.working_dir.join(&sdk_dir_name())
}

fn configure_emscripten_sdk(context: &Context) -> FailableUnit {
    logs::out(log_tag!(), "Configuring Emscripten SDK - this may take a while ...");

    let emsdk = sdk_dir(context).join("emsdk");

    remote_zips::fetch(EMSCRIPTEN_URL, &sdk_dir_name(), &context.working_dir)?;
    scripts::run(&Script::new(&format!("{:?} install {}", &emsdk, EMSCRIPTEN_VERSION)))?;
    scripts::run(&Script::new(&format!("{:?} activate {}", &emsdk, EMSCRIPTEN_VERSION)))
}

fn compile_application(context: &Context) -> FailableUnit {
    logs::out(log_tag!(), "Compiling application ...");

    let emscripten_flags = format!(
        r#"-s EXPORTED_FUNCTIONS='["_main","_fileno"]' -O2 -s USE_SDL=2 -s USE_SDL_IMAGE=2 -s SDL2_IMAGE_FORMATS='["png"]' -s USE_WEBGL2=1 --preload-file {:?}@/assets"#,
        &context.assets_dir
    );

    let mut environment = HashMap::new();
    environment.insert("EMCC_CFLAGS".to_owned(), emscripten_flags);

    let script_prefix = if cfg!(target_os = "windows") {
        "emsdk_env.bat &"
    } else {
        ". ./emsdk_env.sh &&"
    };
    
    scripts::run(
        &Script::new(&format!(
            r#"{} cargo rustc {} --manifest-path {:?} --target wasm32-unknown-emscripten --bin crust --target-dir {:?}"#,
            &script_prefix,
            context.variant.rust_compiler_flag(),
            context.source_dir.join("Cargo.toml"),
            context.rust_build_dir,
        ))
        .working_dir(&sdk_dir(context))
        .environment(&environment),
    )
}

fn create_output(context: &Context) -> FailableUnit {
    logs::out(log_tag!(), "Creating output ...");

    let build_variant_dir = context.rust_build_dir.join("wasm32-unknown-emscripten").join(context.variant.id()).join("deps");

    outputs::clean(context)?;
    outputs::collect(
        context,
        vec![
            build_variant_dir.join("crust.wasm"),
            build_variant_dir.join("crust.data"),
            build_variant_dir.join("crust.js"),
        ],
    )?;

    io::write_string(INDEX_HTML_TEMPLATE, &outputs::output_dir(context).join("index.html"))
}

fn open_in_browser(context: &Context) -> FailableUnit {
    let launch_command = if cfg!(target_os = "windows") { "start" } else { "open" };

    scripts::run(&Script::new(&format!("{} http://localhost:8000", &launch_command)))?;
    scripts::run(&Script::new("http -p 8000").working_dir(&outputs::output_dir(context)))
}

const INDEX_HTML_TEMPLATE: &str = r#"
<html>
    <head>
        <meta charset="utf-8">
        <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
        <style>
            canvas.emscripten {
                display:block;
                border: 0px none;
                background-color: #333333;
            }

            textarea.emscripten {
                resize: none;
                width: 600px;
                height: 200px;
                display:block;
                border: 0px none;
                padding: 8px;
                background-color: #222222;
                color: #ffffff;
                margin: 0px;
            }

            .content {
                border: 1px solid #333333;
                display: inline-block;
            }
        </style>
    </head>

    <body>
        <div class="content">
            <canvas class="emscripten" id="canvas" width="600", height="360" oncontextmenu="event.preventDefault()" tabindex=-1></canvas>
            <textarea class="emscripten" id="output" rows="8"></textarea>
        </div>

        <script type='text/javascript'>
            var Module = {
                preRun: [],
                postRun: [],
                print: (function () {
                    var element = document.getElementById('output');
                    if (element) element.value = '';
                    return function (text) {
                        if (arguments.length > 1) {
                            text = Array.prototype.slice.call(arguments).join(' ');
                        }

                        console.log(text);

                        if (element) {
                            element.value += text + "\n";
                            element.scrollTop = element.scrollHeight;
                        }
                    };
                })(),
                printErr: function (text) {
                    if (arguments.length > 1) {
                        text = Array.prototype.slice.call(arguments).join(' ');
                    }

                    console.error(text);
                },
                canvas: (function () {
                    return document.getElementById('canvas');
                })()
            };
        </script>
        <script async type="text/javascript" src="crust.js"></script>
    </body>
</html>
"#;
