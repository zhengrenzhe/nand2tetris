use std::fs;
use std::io::{Error, Read};
use std::path::Path;

pub struct File {
    pub stem: String,
    pub lines: Vec<String>,
    pub output_dir: String,
}

pub fn read_lines(file_path: &str) -> Result<File, Error> {
    let file_path_target = Path::new(file_path);
    let mut output_dir = String::from("./");

    if let Some(file_name) = file_path_target.file_name() {
        let file_dir = file_path.replace(file_name.to_str().unwrap(), "");
        if let Ok(abs_path) = Path::new(&file_dir).canonicalize() {
            output_dir = String::from(abs_path.to_str().unwrap())
        }
    };

    let stem = match file_path_target.file_stem() {
        Some(val) => String::from(val.to_str().unwrap()),
        None => String::from("untitled"),
    };

    let mut content = String::new();

    let mut file = fs::File::open(file_path)?;

    file.read_to_string(&mut content)?;

    let lines = content.split('\n').map(String::from).collect();

    Ok(File {
        lines,
        stem,
        output_dir,
    })
}

pub fn write_lines(lines: &[String], file_name: &str) -> Result<bool, Error> {
    fs::write(file_name, lines.join("\n"))?;
    Ok(true)
}
