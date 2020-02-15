pub fn pre_process(lines: Vec<String>, remove_blanks_in_code: bool) -> Vec<String> {
    let mut result_lines: Vec<String> = vec![];

    for line in lines {
        // remove blanks on line start and end.
        let mut pure_content = line.trim().to_string();

        // remove blanks in code.
        if remove_blanks_in_code {
            pure_content = pure_content.replacen(" ", "", 10_000_000);
        }

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

        result_lines.push(pure_content.trim().to_string());
    }

    result_lines
}
