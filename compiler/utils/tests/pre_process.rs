extern crate utils;

use utils::pre_process::pre_process;

#[test]
fn test_pre_process() {
    let result = pre_process(
        vec![
            String::from("D = A + C    "),
            String::from("    D = A + C    "),
            String::from("    D = A + C"),
            String::from("    "),
            String::from("//    "),
            String::from(" D = A + C //   "),
            String::from(" (LOOP)   "),
        ],
        true,
    );

    assert_eq!(result.len(), 5);
    assert_eq!(result[0], String::from("D=A+C"));
    assert_eq!(result[1], String::from("D=A+C"));
    assert_eq!(result[2], String::from("D=A+C"));
    assert_eq!(result[3], String::from("D=A+C"));
    assert_eq!(result[4], String::from("(LOOP)"));

    let result2 = pre_process(
        vec![
            String::from("D = A + C    "),
            String::from("    D = A + C    "),
            String::from("    D = A + C"),
            String::from("    "),
            String::from("//    "),
            String::from(" D = A + C //   "),
            String::from(" (LOOP)   "),
        ],
        false,
    );

    assert_eq!(result2.len(), 5);
    assert_eq!(result2[0], String::from("D = A + C"));
    assert_eq!(result2[1], String::from("D = A + C"));
    assert_eq!(result2[2], String::from("D = A + C"));
    assert_eq!(result2[3], String::from("D = A + C"));
    assert_eq!(result2[4], String::from("(LOOP)"));
}
