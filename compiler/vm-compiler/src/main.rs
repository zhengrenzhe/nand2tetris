use std::io::Error;
use std::process;

use code_gen::code_gen;
use utils::args::get_args;
use utils::io::read_lines;
use utils::pre_process::pre_process;

mod code_gen;
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
    let files = get_args()?;

    for file in files {
        let lines = read_lines(&file)?;

        let clean_codes = pre_process(lines.lines, false);

        let result_codes = code_gen(clean_codes);

        println!("{:?}", result_codes);
    }

    Ok(())
}
