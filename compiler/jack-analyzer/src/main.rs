use std::io::Error;
use std::process;

use code_gen::code_gen;
use parser::parser;
use pre_process::pre_process;
use utils::args::get_args;
use utils::io::read_files;
use xml::{write_ast_xml, write_tokens_xml};

use tokenizer::tokenizer;

mod code_gen;
mod constant;
mod index;
mod node;
mod parser;
mod pre_process;
mod symbol_table;
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
            let ast = parser(&tokens);
            write_ast_xml(&ast, &format!("{}/{}2.xml", file.output_dir, file.stem))?;
            code_gen(&ast);
        }
    }

    Ok(())
}
