use bytes::Bytes;
use std::fs;
use std::io::{Cursor, Read, Seek};
use std::path::PathBuf;
use tracing::info;
use zip::ZipArchive;

pub fn download_zip_archive<S: AsRef<str>>(
    url: S,
) -> Result<ZipArchive<Cursor<Bytes>>, anyhow::Error> {
    let response = reqwest::blocking::get(url.as_ref())?.error_for_status()?;
    let content = Cursor::new(response.bytes()?);
    let archive = zip::ZipArchive::new(content)?;
    Ok(archive)
}

pub fn extract_zip_archive<R: Read + Seek>(
    mut archive: ZipArchive<R>,
    extract_path: PathBuf,
) -> Result<(), anyhow::Error> {
    if !extract_path.is_dir() {
        info!("Creating directory {}", extract_path.display());
        fs::create_dir(extract_path.as_path())?;
    }
    archive.extract(extract_path)?;
    Ok(())
}
