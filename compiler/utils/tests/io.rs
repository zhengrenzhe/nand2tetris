extern crate utils;

use utils::{read_lines, write_lines};

const TEST_FILE_PATH: &str = "tests/test.txt";

fn get_lines() -> Vec<String> {
    vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("1"),
        String::from("2"),
        String::from("3"),
    ]
}

#[test]
fn test_write_lines() {
    let lines = get_lines();

    let write_result = write_lines(&lines, TEST_FILE_PATH);

    match write_result {
        Ok(result) => assert_eq!(result, true),
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_read_lines() {
    let lines = get_lines();

    let read_result = read_lines(TEST_FILE_PATH);

    match read_result {
        Ok(file) => {
            assert_eq!(file.file_name, String::from("test.txt"));
            assert_eq!(file.stem, String::from("test"));
            assert_eq!(file.extension, String::from("txt"));
            assert_eq!(file.lines.len(), lines.len());
            for (index, content) in file.lines.iter().enumerate() {
                assert_eq!(*content, lines[index]);
            }
        }
        Err(err) => panic!("{}", err),
    }
}
