use std::io::Error;
use std::process;

use utils::args::get_args;
use utils::io::read_lines;
use utils::pre_process::pre_process;

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

        println!("{:?}", clean_codes);
    }

    Ok(())
}
