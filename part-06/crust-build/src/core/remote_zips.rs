use crate::{
    core::{downloads, failable_unit::FailableUnit, io, logs},
    log_tag,
};
use std::{fs::DirEntry, path::PathBuf};

pub fn fetch(url: &str, destination_dir_name: &str, destination_parent_dir: &PathBuf) -> FailableUnit {
    let target_dir = destination_parent_dir.join(destination_dir_name);

    if target_dir.exists() {
        logs::out(log_tag!(), &format!("Destination already exists, skipping download: {:?}", &target_dir));
        return Ok(());
    }

    io::in_temp_dir(&mut |temp_dir| {
        let download_file_path = temp_dir.to_path_buf().join("download.zip");
        downloads::download(&url, &download_file_path)?;

        let unzipped_dir = temp_dir.join("unzipped");
        io::unzip(&download_file_path, &unzipped_dir)?;

        // We will now massage the name of the unzipped directory to be whatever the caller specified. The directory to rename will be the first child of the 'unzipped' directory where we just unzipped the files.
        let content_dir = unzipped_dir.join(&destination_dir_name);
        io::rename(&std::fs::read_dir(&unzipped_dir)?.filter_map(|e| e.ok()).collect::<Vec<DirEntry>>()[0].path(), &content_dir)?;
        io::create_dir(&destination_parent_dir)?;
        io::copy(&content_dir, &destination_parent_dir)
    })
}
