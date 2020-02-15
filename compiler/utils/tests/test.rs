extern crate utils;

use utils::{read_lines, write_lines};

#[test]
fn io() {
    let test_file_path = "tests/test.txt";

    let lines = vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("1"),
        String::from("2"),
        String::from("3"),
    ];

    let write_result = write_lines(&lines, test_file_path);

    match write_result {
        Ok(result) => assert_eq!(result, true),
        Err(err) => panic!("{}", err),
    }

    let read_result = read_lines(test_file_path);

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
