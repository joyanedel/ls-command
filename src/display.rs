use colored::Colorize;
use std::cmp::max;

use crate::structs::ControlledEntry;

pub fn display_entries(entries: Vec<&ControlledEntry>, long_format: bool) {
    let max_length = longest_name_size(&entries);

    println!(
        "{:<width$} {:>10}",
        "Filename".bold(),
        "Size".bold(),
        width = max_length + 6
    );
    for entry in entries {
        display_entry(entry, long_format, max_length + 2);
    }
}

fn display_entry(entry: &ControlledEntry, long_format: bool, width: usize) -> bool {
    println!(
        " -> {:<width$} {:>10}",
        entry.name,
        entry.size,
        width = width
    );

    long_format
}

fn longest_name_size(entries: &Vec<&ControlledEntry>) -> usize {
    entries
        .iter()
        .map(|e| e.name.len())
        .reduce(|acc, e| max(acc, e))
        .unwrap()
}
