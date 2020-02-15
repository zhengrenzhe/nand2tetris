pub fn pre_process(lines: Vec<String>) -> Vec<String> {
    let mut result_lines: Vec<String> = vec![];

    for line in lines {
        // remove blanks on line start and end.
        let mut pure_content = line.trim().replacen(" ", "", 10_000_000);

        // if line starts with / or length equals 0(empty line), then ignore it.
        if pure_content.starts_with('/') || pure_content.is_empty() {
            continue;
        }

        // remove comment in code line
        if let Some(index) = pure_content.find('/') {
            if let Some(str) = pure_content.get(0..index) {
                pure_content = String::from(str)
            }
        }

        result_lines.push(pure_content);
    }

    result_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pre_process() {
        let result = pre_process(vec![
            String::from("D = A + C    "),
            String::from("    D = A + C    "),
            String::from("    D = A + C"),
            String::from("    "),
            String::from("//    "),
            String::from(" D = A + C //   "),
            String::from(" (LOOP)   "),
        ]);

        assert_eq!(result.len(), 5);
        assert_eq!(result[0], String::from("D=A+C"));
        assert_eq!(result[1], String::from("D=A+C"));
        assert_eq!(result[2], String::from("D=A+C"));
        assert_eq!(result[3], String::from("D=A+C"));
        assert_eq!(result[4], String::from("(LOOP)"));
    }
}
