extern crate argparse_rs;

use argparse_rs::{ArgParser, ArgType};
use std::{
    env,
    fs::{DirEntry, ReadDir},
    os::unix::fs::{MetadataExt, PermissionsExt},
};

fn main() {
    let mut parser = ArgParser::new("ls".into());
    configure_parser(&mut parser);

    let args: Vec<String> = env::args().collect();

    let parsed_arguments = parse_args(&args, &parser);
    let pwd = env::current_dir().unwrap();

    let paths = pwd.read_dir().unwrap();

    if parsed_arguments.help {
        parser.help();
        return;
    }

    list_elements(paths, &parsed_arguments);
}

fn configure_parser(parser: &mut ArgParser) {
    parser.add_opt(
        "long",
        Some("false"),
        'l',
        false,
        "Show output in long format",
        ArgType::Flag,
    );
    parser.add_opt(
        "all",
        Some("false"),
        'a',
        false,
        "Show all entries, even hidden ones",
        ArgType::Flag,
    );
}

fn parse_args(args_list: &Vec<String>, parser: &ArgParser) -> Args {
    let p_res = parser.parse(args_list.iter(), false).unwrap();

    let arg_long = p_res.get("long").unwrap();
    let arg_hidden = p_res.get("all").unwrap();
    let arg_help = p_res.get("help").unwrap();

    return Args {
        long: arg_long,
        hidden: arg_hidden,
        help: arg_help,
    };
}

fn list_elements(entries: ReadDir, args: &Args) {
    if args.long {
        println!("Perms\tUID\tSize\tFilename")
    }

    for path in entries {
        let file_entry = path.unwrap();
        let is_hidden_entry = file_entry.file_name().to_str().unwrap().starts_with(".");
        if is_hidden_entry && !args.hidden {
            continue;
        }

        display_entry(&file_entry, args.long)
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

struct Args {
    long: bool,
    hidden: bool,
    help: bool,
}
