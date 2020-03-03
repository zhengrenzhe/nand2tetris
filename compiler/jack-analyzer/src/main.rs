use std::io::Error;
use std::process;

use utils::args::get_args;
use utils::io::read_file;

mod constant;
mod tokenizer;

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
        let content = read_file(&file_path)?;
        println!("{}", content);
    }

    Ok(())
}
