use crate::core::{failable::Failable, failable_unit::FailableUnit, io, script::Script};
use std::{
    path::Path,
    process::{Command, Stdio},
};

#[cfg(not(target_os = "windows"))]
fn create_command(script_dir: &Path, script_block: &str) -> Failable<Command> {
    let mut script_content = "#!/usr/bin/env bash\n\n".to_owned();
    script_content += "set -o errexit\n";
    script_content += "set -o pipefail\n";
    script_content += "set -o nounset\n\n";
    script_content += script_block;

    let script_file_path = script_dir.join("script.sh");
    io::write_string(&script_content, &script_file_path)?;
    io::apply_permissions(&script_file_path, 0o755)?;

    Ok(Command::new(&script_file_path))
}

#[cfg(target_os = "windows")]
fn create_command(script_dir: &Path, script_block: &str) -> Failable<Command> {
    let script_file_path = script_dir.join("script.bat");
    io::write_string(script_block, &script_file_path)?;

    Ok(Command::new(&script_file_path))
}

pub fn run(script: &Script) -> FailableUnit {
    io::in_temp_dir(&mut |temp_dir| {
        let mut command = create_command(temp_dir, &script.content)?;

        command.current_dir(match &script.working_dir {
            Some(working_dir) => working_dir.clone(),
            _ => temp_dir.to_path_buf(),
        });

        command.envs(&script.environment);
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());

        let output = command.output()?;
        let status_code = output.status.code().ok_or("Failed to get shell script status code.")?;

        if output.status.success() {
            Ok(())
        } else {
            Err(format!("Shell script returned failed status code: {:?}", &status_code).into())
        }
    })
}
