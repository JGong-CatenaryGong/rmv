use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct DirEntry {
    pub path: String,
    pub is_dir: bool, // 是否为文件夹
}

pub fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(DirEntry)) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            }
            cb(DirEntry {
                path: path.to_string_lossy().into_owned(),
                is_dir: path.is_dir(),
            });
        }
    }
    Ok(())
}
