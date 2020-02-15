use std::fs;
use std::io::{Error, Read};
use std::path::Path;

pub struct File {
    pub file_name: String,
    pub stem: String,
    pub extension: String,
    pub size: usize,
    pub lines: Vec<String>,
}

pub fn read_lines(file_path: &str) -> Result<File, Error> {
    let file_path_target = Path::new(file_path);

    let file_name = match file_path_target.file_name() {
        Some(val) => String::from(val.to_str().unwrap()),
        None => String::from("untitled.out"),
    };

    let stem = match file_path_target.file_stem() {
        Some(val) => String::from(val.to_str().unwrap()),
        None => String::from("untitled"),
    };

    let extension = match file_path_target.extension() {
        Some(val) => String::from(val.to_str().unwrap()),
        None => String::from("out"),
    };

    let mut content = String::new();

    let mut file = fs::File::open(file_path)?;

    let size = file.read_to_string(&mut content)?;

    let lines = content.split('\n').map(String::from).collect();

    Ok(File {
        size,
        lines,
        stem,
        file_name,
        extension,
    })
}

pub fn write_lines(lines: &[String], file_name: &str) -> Result<bool, Error> {
    fs::write(file_name, lines.join("\n"))?;
    Ok(true)
}
