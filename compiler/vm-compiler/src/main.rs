use std::io::Error;
use std::process;

use code_gen::code_gen;
use utils::args::get_args;
use utils::io::{read_lines, write_lines};
use utils::pre_process::pre_process;

mod code_gen;
mod instructions;
mod lexical;

fn main() {
    match run() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1)
        }
    }
}

fn run() -> Result<(), Error> {
    let file_paths = get_args()?;

    for file_path in file_paths {
        let file = read_lines(&file_path)?;

        let clean_codes = pre_process(file.lines, false);

        let result_codes = code_gen(clean_codes);

        write_lines(&result_codes, format!("{}.asm", file.stem).as_str())?;
    }

    Ok(())
}
