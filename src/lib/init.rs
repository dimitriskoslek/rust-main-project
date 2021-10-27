use super::error::SocError;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[allow(dead_code)]
pub fn init() -> Result<(), SocError> {
    let dir = Path::new("sociality");

    fs::create_dir(dir)?;
    fs::create_dir(dir.join("objects"))?;
    fs::create_dir(dir.join("refs"))?;
    fs::create_dir(dir.join("refs").join("heads"))?;

    let mut head = File::create(dir.join("HEAD"))?;
    head.write_all("refs: refs/head/master".as_bytes())?;

    Ok(())
}

#[allow(dead_code)]
pub fn test() -> Result<(), SocError> {
    Ok(())
}

#[allow(dead_code)]
pub fn check() -> Result<(), SocError> {
    Ok(())
}