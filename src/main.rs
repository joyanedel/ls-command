extern crate argparse_rs;

use argparse_rs::{ArgParser, ArgType};
use std::{env, fs::ReadDir};

fn main() {
    let mut parser = ArgParser::new("argparse".into());
    configure_parser(&mut parser);

    let args: Vec<String> = env::args().collect();

    let parsed_arguments = parse_args(&args, &parser);
    let pwd = env::current_dir().unwrap();

    let paths = pwd.read_dir().unwrap();

    list_elements(paths, parsed_arguments.hidden);
}

fn parse_args(args_list: &Vec<String>, parser: &ArgParser) -> Args {
    let p_res = parser.parse(args_list.iter()).unwrap();

    let arg_long = p_res.get("long").unwrap();
    let arg_hidden = p_res.get("hidden").unwrap();

    return Args {
        long: arg_long,
        hidden: arg_hidden,
    };
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
        "hidden",
        Some("false"),
        'h',
        false,
        "Show hidden entries",
        ArgType::Flag,
    );
}

fn list_elements(entries: ReadDir, show_hidden_entries: bool) {
    for path in entries {
        let file_entry = path.unwrap();
        let is_hidden_entry = file_entry.file_name().to_str().unwrap().starts_with(".");
        if is_hidden_entry && !show_hidden_entries {
            continue;
        }

        println!("{:?}", file_entry.file_name());
    }
}

#[derive(Debug)]
struct Args {
    long: bool,
    hidden: bool,
}
