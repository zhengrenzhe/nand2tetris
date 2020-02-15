use std::io::Error;
use std::process;

use utils::args::get_args;
use utils::io::read_lines;

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

    println!("{}", file.lines.len());

    Ok(())
}
