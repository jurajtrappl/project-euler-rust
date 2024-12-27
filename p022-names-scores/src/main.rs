#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::fs::File;
use std::io::Read;

fn read_input_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut buffer = String::new();
    _ = file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn alphabetical_score(value: &str) -> u32 {
    value.chars().map(|c| c.to_ascii_lowercase() as u32 - 96).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input_file("input")?;

    let cleaned = input.replace("\"", "");
    let mut names: Vec<&str> = cleaned.split(",").collect();
    names.sort();

    let names_scores: u32 = names
        .iter()
        .enumerate()
        .map(|(i, name)| ((i as u32) + 1) * alphabetical_score(&name))
        .sum();
    println!("{}", names_scores);

    Ok(())
}
