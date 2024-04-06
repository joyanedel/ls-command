extern crate argparse_rs;

use argparse_rs::{ArgParser, ArgType};
use std::env;
// use std::collections::HashMap;

fn main() {
    let mut parser = ArgParser::new("argparse".into());
    parser.add_opt(
        "long",
        Some("false"),
        'l',
        false,
        "Show output in long format",
        ArgType::Flag,
    );

    let args: Vec<String> = env::args().collect();

    let parsed_arguments = parse_args(&args, &parser);
    println!("{:?}", parsed_arguments);
}

fn parse_args(args_list: &Vec<String>, parser: &ArgParser) -> Args {
    let p_res = parser.parse(args_list.iter()).unwrap();

    let arg_long = p_res.get("long").unwrap();

    return Args { long: arg_long };
}

#[derive(Debug)]
struct Args {
    long: bool,
}
