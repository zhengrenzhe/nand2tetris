// remove all blank char, blank lines, comments.

pub fn pre_process(source: String) -> Vec<String> {
    let mut lines: Vec<String> = vec![];

    for line in source.lines() {
        // remove blanks on line start and end.
        let mut pure_content = line.trim().to_string();

        // if line starts with / or length equals 0(empty line), then ignore it.
        if pure_content.starts_with('/') || pure_content.is_empty() {
            continue;
        }

        // remove comment in code line
        if let Some(index) = pure_content.find('/') {
            if let Some(str) = pure_content.get(0..index) {
                pure_content = str.to_string();
            }
        }

        // remove blank char
        pure_content = pure_content.replacen(" ", "", 1000);

        lines.push(pure_content);
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pre_process() {
        let result = pre_process("\t aaa \nbbb\n //\n\n\n ccc//dc\n\nD = A+ C".to_string());
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], "aaa".to_string());
        assert_eq!(result[1], "bbb".to_string());
        assert_eq!(result[2], "ccc".to_string());
        assert_eq!(result[3], "D=A+C".to_string());
    }
}
