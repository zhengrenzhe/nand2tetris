use crate::constant::{is_empty_character, is_split_character, match_token, Token};

pub fn tokenizer(content: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut last_chars: Vec<String> = vec![];
    let mut in_string = false;

    for char in content.chars() {
        if char == '"' {
            if in_string {
                in_string = false;
            } else {
                in_string = true;
            }
        }

        if !in_string {
            if is_split_character(char) {
                let tk: String = last_chars.join("");
                if tk != "" {
                    tokens.push(match_token(&tk));
                }
                if char != ' ' {
                    tokens.push(match_token(&char.to_string()))
                }
                last_chars.clear();
                continue;
            }

            if is_empty_character(char) {
                continue;
            }
        }

        last_chars.push(char.to_string());
    }

    tokens
}
