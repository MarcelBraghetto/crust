use crate::{
    core::{failable_unit::FailableUnit, io, logs},
    log_tag,
};
use std::path::Path;

pub fn download(url: &str, destination: &Path) -> FailableUnit {
    logs::out(log_tag!(), &format!("Download: {:?}", url));
    logs::out(log_tag!(), &format!("Into: {:?}", destination));

    let response = reqwest::blocking::get(url)?;

    if !response.status().is_success() {
        return Err("Url request was not successful.".into());
    }

    let content = response.bytes()?;

    io::write_bytes(&content, &destination.to_path_buf())?;
    logs::out(log_tag!(), &format!("Download complete: {:?}", destination));

    Ok(())
}
