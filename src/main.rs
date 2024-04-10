mod parser;

extern crate argparse_rs;

use argparse_rs::ArgType;
use parser::ProgramArg;
use std::{
    collections::HashMap,
    env,
    fs::{DirEntry, ReadDir},
    os::unix::fs::{MetadataExt, PermissionsExt},
    process::exit,
};

fn main() {
    let args: Vec<ProgramArg> = vec![
        ProgramArg {
            name: "long",
            default: Some("false"),
            flag: 'l',
            required: false,
            help: "Show output in long format",
            type_: ArgType::Flag,
        },
        ProgramArg {
            name: "all",
            default: Some("false"),
            flag: 'a',
            required: false,
            help: "Show all entries, even hidden ones",
            type_: ArgType::Flag,
        },
    ];
    let (parser, parsed_args) = parser::create_parser(String::from("ls-command"), args);

    if *parsed_args.get("help").unwrap() {
        parser.help();
        exit(0);
    }

    let pwd = env::current_dir().unwrap();
    let paths = pwd.read_dir().unwrap();

    list_elements(paths, &parsed_args);
}

#[warn(dead_code)]
fn list_elements(entries: ReadDir, args: &HashMap<String, bool>) {
    let is_long_format = *args.get("long").unwrap();
    let is_hidden_entries_allowed = *args.get("all").unwrap();

    if is_long_format {
        println!("Perms\tUID\tSize\tFilename")
    }

    for path in entries {
        let file_entry = path.unwrap();
        let is_hidden_entry = file_entry.file_name().to_str().unwrap().starts_with(".");
        if is_hidden_entry && !is_hidden_entries_allowed {
            continue;
        }

        display_entry(&file_entry, is_long_format)
    }
}

fn display_entry(entry: &DirEntry, long_format: bool) {
    let is_dir = entry.file_type().unwrap().is_dir();
    let suffix = if is_dir { "/" } else { "" };
    if long_format {
        let metadata = entry.metadata().unwrap();
        let permissions = metadata.permissions().mode();
        let uid = metadata.uid();
        let size_file = metadata.size();
        println!(
            "{:?}\t{}\t{}\t{}{}",
            permissions,
            uid,
            size_file,
            entry.file_name().to_str().unwrap(),
            suffix
        )
    } else {
        println!("{}{}", entry.file_name().to_str().unwrap(), suffix)
    }
}
