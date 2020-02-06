use std::env;
use std::fs;
use std::io::{Error, ErrorKind, Read};

pub fn read_file() -> Result<(String, String), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::new(
            ErrorKind::NotFound,
            "do not have enough arguments",
        ));
    }

    let file_path = args[1].as_str();

    let mut file_name: Vec<&str> = file_path.split('/').collect();
    let file_name = file_name.pop().unwrap();
    let file_name: Vec<&str> = file_name.split('.').collect();
    let file_name = file_name[0];

    let mut content = String::new();

    let mut file = fs::File::open(file_path)?;

    let _ = file.read_to_string(&mut content)?;

    Ok((content, file_name.to_string()))
}

pub fn write_file(lines: Vec<String>, file_name: String) -> Result<(), Error> {
    fs::write(file_name, lines.join("\n"))?;
    Ok(())
}
