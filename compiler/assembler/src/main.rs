use std::io::Error;
use std::process;

use pre_process::pre_process;
use symbol_table::generate_symbol_table;
use translate::translate;
use utils::args::get_args;
use utils::io::{read_lines, write_lines};

mod pre_process;
mod symbol_table;
mod translate;

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

    for f in file_path {
        let file = read_lines(&f)?;

        let clean_codes = pre_process(file.lines);
        let (clean_codes, mut symbol_table) = generate_symbol_table(clean_codes);
        let clean_codes = translate(clean_codes, &mut symbol_table);

        write_lines(&clean_codes, format!("{}.hack", file.stem).as_str())?;
    }

    Ok(())
}
