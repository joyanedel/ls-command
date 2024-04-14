use std::{env::current_dir, path::PathBuf};

use crate::structs::ControlledEntry;

pub fn get_absolute_working_dir(working_dir: String) -> Result<PathBuf, ()> {
    let mut dir = get_current_dir();
    dir.push(working_dir);

    if !dir.exists() {
        return Err(());
    }

    if dir.is_file() {
        Ok(dir.parent().unwrap().to_path_buf())
    } else {
        Ok(dir)
    }
}

fn get_current_dir() -> PathBuf {
    current_dir().unwrap()
}

pub fn get_directory_entries(dir: PathBuf) -> Vec<ControlledEntry> {
    let entries = dir.read_dir().unwrap();
    entries.map(|v| ControlledEntry::from(v.unwrap())).collect()
}
