use std::fs;
use std::io::{Error, Read};
use std::path::Path;
use std::result::Result::Ok;

pub struct File {
    pub stem: String,
    pub lines: Vec<String>,
    pub output_dir: String,
    pub insert_bootstrap: bool,
}

pub fn read_lines(file_path: &str) -> Result<File, Error> {
    let file_path_target = Path::new(file_path);
    let mut output_dir = String::from("./");
    let mut content = String::new();
    let mut insert_bootstrap = false;

    if file_path_target.is_dir() {
        output_dir = String::from(file_path_target.to_str().unwrap());
        insert_bootstrap = true;
        let mut vm_files: Vec<String> = vec![];
        for item in file_path_target.read_dir().expect("read folder error") {
            if let Ok(child) = item {
                let child_file_path = String::from(child.path().to_str().unwrap());
                if child_file_path.contains(".vm") {
                    vm_files.push(child_file_path)
                }
            }
        }
        for file_path in vm_files {
            let mut part_content = String::new();
            let mut file = fs::File::open(file_path)?;
            file.read_to_string(&mut part_content)?;
            content.push_str(&part_content);
        }
    } else {
        if let Some(file_name) = file_path_target.file_name() {
            let file_dir = file_path.replace(file_name.to_str().unwrap(), "");
            if let Ok(abs_path) = Path::new(&file_dir).canonicalize() {
                output_dir = String::from(abs_path.to_str().unwrap())
            }
        };

        let mut file = fs::File::open(file_path)?;

        file.read_to_string(&mut content)?;
    }

    let stem = match file_path_target.file_stem() {
        Some(val) => String::from(val.to_str().unwrap()),
        None => String::from("untitled"),
    };

    let lines = content.split('\n').map(String::from).collect();

    Ok(File {
        lines,
        stem,
        output_dir,
        insert_bootstrap,
    })
}

pub fn write_lines(lines: &[String], file_name: &str) -> Result<bool, Error> {
    fs::write(file_name, lines.join("\n"))?;
    Ok(true)
}
