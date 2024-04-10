extern crate argparse_rs;

use std::{collections::HashMap, env};

use argparse_rs::{ArgParseResults, ArgParser, ArgType};

type CreateParserReturn = (ArgParser, ParsedArgs);
pub fn create_parser(program_name: String, args: Vec<ProgramArg>) -> CreateParserReturn {
    let mut parser = ArgParser::new(program_name);
    configure_parser(&mut parser, &args);

    let runtime_args = get_list_of_args(&parser);
    let parsed_args = parse_args(runtime_args, &args);

    (parser, parsed_args)
}

fn configure_parser(parser: &mut ArgParser, arguments: &Vec<ProgramArg>) {
    for argument in arguments {
        parser.add_opt(
            argument.name,
            argument.default,
            argument.flag,
            argument.required,
            argument.help,
            argument.type_.clone(),
        );
    }
}

fn get_list_of_args(parser: &ArgParser) -> ArgParseResults {
    match parser.parse(env::args().collect::<Vec<String>>().iter(), false) {
        Err(e) => panic!("{}", e),
        Ok(v) => v,
    }
}

fn parse_args(runtime_args: ArgParseResults, p_args: &Vec<ProgramArg>) -> ParsedArgs {
    let mut args = ParsedArgs::new();
    args.insert("help".to_string(), runtime_args.get("help").unwrap());

    for arg in p_args {
        let name = arg.name;
        let value = match runtime_args.get(name) {
            Some(v) => v,
            None => false,
        };

        args.insert(name.into(), value);
    }

    args
}

pub struct ProgramArg<'a> {
    pub name: &'a str,
    pub default: Option<&'a str>,
    pub flag: char,
    pub required: bool,
    pub help: &'a str,
    pub type_: ArgType,
}

type ParsedArgs = HashMap<String, bool>;
