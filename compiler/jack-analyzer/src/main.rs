use std::io::Error;
use std::process;

use pre_process::pre_process;
use utils::args::get_args;
use utils::io::read_files;
use xml::write_tokens_xml;

use tokenizer::tokenizer;

mod constant;
mod pre_process;
mod tokenizer;
mod xml;

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
        for file in read_files(&file_path, ".jack")? {
            let tokens = tokenizer(&pre_process(file.content));
            write_tokens_xml(&tokens, &format!("{}/{}T2.xml", file.output_dir, file.stem))?;
        }
    }

    Ok(())
}
