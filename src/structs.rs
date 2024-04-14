use std::{
    fs::{DirEntry, FileType},
    os::unix::fs::MetadataExt,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "0.1", about = "ls is a list directory tool")]
pub struct ProgramArgs {
    #[arg(short, long)]
    pub all: bool,
    #[arg(short, long)]
    pub long: bool,
    #[arg(default_value = ".")]
    pub working_dir: String,
}

#[derive(Debug)]
pub struct ControlledEntry {
    pub name: String,
    pub size: u64,
    pub entry_type: EntryType,
    pub is_hidden: bool,
}

#[derive(Debug)]
pub enum EntryType {
    FILE,
    DIRECTORY,
}

impl From<FileType> for EntryType {
    fn from(value: FileType) -> Self {
        if value.is_file() {
            Self::FILE
        } else {
            Self::DIRECTORY
        }
    }
}

impl From<DirEntry> for ControlledEntry {
    fn from(value: DirEntry) -> Self {
        ControlledEntry {
            name: value.file_name().into_string().unwrap(),
            size: value.metadata().unwrap().size(),
            entry_type: EntryType::from(value.file_type().unwrap()),
            is_hidden: value.file_name().into_string().unwrap().starts_with("."),
        }
    }
}
