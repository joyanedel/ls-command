use clap::Parser;

mod directory;
mod display;
mod structs;

fn main() {
    let args = structs::ProgramArgs::parse();
    let wd = directory::get_absolute_working_dir(args.working_dir);
    let entries = directory::get_directory_entries(wd.unwrap());

    let result: Vec<_> = entries
        .iter()
        .filter(|&v| args.all || !v.is_hidden)
        .collect();

    display::display_entries(result, args.long)
}
