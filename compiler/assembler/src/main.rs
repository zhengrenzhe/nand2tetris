use std::io::Error;
use std::process;

use pre_process::pre_process;
use symbol_table::generate_symbol_table;
use utils::args::get_args;
use utils::io::read_lines;

mod pre_process;
mod symbol_table;

fn main() {
    match run() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}

fn run() -> Result<(), Error> {
    let file_path = get_args()?;
    let file = read_lines(&file_path)?;

    let clean_codes = pre_process(file.lines);
    let (clean_codes, symbol_table) = generate_symbol_table(clean_codes);

    println!("{:?}", clean_codes);
    println!("{:?}", symbol_table);

    Ok(())
}
