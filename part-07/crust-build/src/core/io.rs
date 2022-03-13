use crate::{
    core::{failable_unit::FailableUnit, logs, script::Script, scripts},
    log_tag,
};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

pub fn create_symlink(source: &PathBuf, target: &PathBuf, working_dir: &PathBuf) -> FailableUnit {
    delete(target)?;

    logs::out(log_tag!(), &format!("Creating symlink: {:?} <=> {:?}", source, target));

    if cfg!(target_os = "windows") {
        scripts::run(&Script::new(&format!("mklink /D {:?} {:?}", &target, &source)).working_dir(&working_dir))
    } else {
        scripts::run(&Script::new(&format!("ln -s {:?} {:?}", &source, &target)).working_dir(&working_dir))
    }
}

pub fn in_temp_dir(work: &mut dyn FnMut(&Path) -> FailableUnit) -> FailableUnit {
    let temp = tempfile::Builder::new().prefix("crust").tempdir()?;
    work(&temp.path())
}

pub fn write_bytes(content: &[u8], path: &PathBuf) -> FailableUnit {
    let parent = path.parent().ok_or("Parent directory not found")?;
    std::fs::create_dir_all(parent)?;

    let mut file = File::create(path)?;
    file.write_all(content)?;

    Ok(())
}

pub fn write_string(content: &str, path: &PathBuf) -> FailableUnit {
    write_bytes(content.as_bytes(), path)?;
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn apply_permissions(_: &PathBuf, _: u32) -> FailableUnit {
    // On Windows this is a no-op but we will leave it stubbed so we can cross compile easily.
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn apply_permissions(path: &PathBuf, permissions: u32) -> FailableUnit {
    use std::os::unix::fs::PermissionsExt;
    Ok(std::fs::set_permissions(&path, std::fs::Permissions::from_mode(permissions))?)
}

pub fn copy(source: &PathBuf, destination: &PathBuf) -> FailableUnit {
    if source.is_dir() {
        if cfg!(target_os = "windows") {
            scripts::run(&Script::new(&format!(
                "xcopy /E /H /I {:?} {:?}",
                &source,
                &destination.join(source.file_name().ok_or("Missing file name")?)
            )))?;
        } else {
            scripts::run(&Script::new(&format!("cp -R {:?} {:?}", &source, &destination)))?;
        }
    } else {
        std::fs::copy(source, destination)?;
    }

    Ok(())
}

pub fn create_dir(path: &PathBuf) -> FailableUnit {
    std::fs::create_dir_all(path)?;

    Ok(())
}

pub fn delete(victim: &PathBuf) -> FailableUnit {
    logs::out(log_tag!(), &format!("Deleting {:?}", victim));

    // We need to also check if we are trying to delete a symlink by querying for link meta data.
    // The regular `.exists` method will return false even if the victim is a broken symlink.
    let is_symlink = std::fs::read_link(&victim).is_ok();

    if !is_symlink && !victim.exists() {
        return Ok(());
    }

    if victim.is_dir() || is_symlink {
        std::fs::remove_dir_all(victim)?;
    } else {
        std::fs::remove_file(victim)?;
    }

    Ok(())
}

pub fn rename(source: &PathBuf, destination: &PathBuf) -> FailableUnit {
    logs::out(log_tag!(), &format!("Renaming: {:?} => {:?}", source, destination));

    std::fs::rename(source, destination)?;
    Ok(())
}

pub fn unzip(source: &PathBuf, destination: &PathBuf) -> FailableUnit {
    logs::out(log_tag!(), &format!("Unzipping: {:?} => {:?}", source, destination));

    create_dir(destination)?;

    let zip_file = File::open(&source)?;
    let mut archive = zip::ZipArchive::new(zip_file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => destination.join(path.to_owned()),
            None => continue,
        };

        if (&*file.name()).ends_with('/') {
            create_dir(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    create_dir(&parent.to_path_buf())?;
                }
            }

            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        // If the file has any Unix permissions we want to retain them, on Windows this would be a no-op.
        if let Some(mode) = file.unix_mode() {
            apply_permissions(&outpath, mode)?;
        }
    }

    Ok(())
}
