pub fn pre_process(file_content: String) -> String {
    let lines: Vec<String> = file_content.split('\n').map(String::from).collect();
    let mut result: Vec<String> = vec![];
    for line in lines {
        let pure_content = line.trim();
        if pure_content.starts_with("//") {
            continue;
        }
        if let Some(index) = pure_content.find("//") {
            result.push(String::from(&(pure_content.to_string())[..index]));
        } else {
            result.push(String::from(pure_content));
        }
    }

    let result: String = result.join("\n");

    let mut res: Vec<String> = vec![];
    let mut prev_char = ' ';
    let mut ignore = false;

    for c in result.chars() {
        if prev_char == '/' && c == '*' {
            res.pop();
            res.pop();
            ignore = true;
            prev_char = c;
            continue;
        }

        if prev_char == '*' && c == '/' {
            ignore = false;
            prev_char = c;
            continue;
        }

        if ignore {
            prev_char = c;
            continue;
        }

        res.push(c.to_string());
        prev_char = c;
    }

    res.join("")
}
