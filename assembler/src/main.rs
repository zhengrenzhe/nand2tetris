use std::io::Error;
use std::process;

use crate::io::{read_file, write_file};
use crate::pre_process::pre_process;
use crate::translate::translate;

mod io;
mod pre_process;
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
    let (source, file_name) = read_file()?;

    // remove all blank lines, comments
    let pre_processed_code = pre_process(source);

    let result = translate(pre_processed_code);

    write_file(result, format!("{}.hack", file_name))?;

    Ok(())
}
