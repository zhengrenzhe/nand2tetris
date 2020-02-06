use crate::symbol_table::SymbolTable;

// remove all blank char, blank lines, comments.
pub fn pre_process(source: String) -> (Vec<String>, SymbolTable) {
    let mut lines: Vec<String> = vec![];
    let mut segment_labels = SymbolTable::new();
    let mut line_number = 0;

    for line in source.lines() {
        // remove blanks on line start and end.
        let mut pure_content = line.trim().to_string().replacen(" ", "", 1000);

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

        // (LABEL_NAME) segment label
        if pure_content.starts_with('(') {
            pure_content = pure_content.replace("(", "").replace(")", "");
            segment_labels.set(pure_content, format!("0{:015b}", line_number));
            continue;
        }

        lines.push(pure_content);
        line_number += 1;
    }

    (lines, segment_labels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pre_process() {
        let (result, mut segment_table) = pre_process(
            "\t aaa \nbbb\n //\n\n\n ccc//dc\n\nD = A+ C\n (LOOP) \n @LOOP \n (END)\n @END\n\n "
                .to_string(),
        );

        assert_eq!(result.len(), 6);
        assert_eq!(result[0], "aaa".to_string());
        assert_eq!(result[1], "bbb".to_string());
        assert_eq!(result[2], "ccc".to_string());
        assert_eq!(result[3], "D=A+C".to_string());
        assert_eq!(result[4], "@LOOP".to_string());
        assert_eq!(result[5], "@END".to_string());

        assert_eq!(segment_table.get("LOOP"), "0000000000000100");
        assert_eq!(segment_table.get("END"), "0000000000000101");
    }
}
