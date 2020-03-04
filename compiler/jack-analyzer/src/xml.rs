use std::io::Error;

use crate::constant::Token;
use utils::io::write_lines;

pub fn write_tokens_xml(tokens: &[Token], file_path: &str) -> Result<bool, Error> {
    let mut lines: Vec<String> = vec![];
    lines.push("<tokens>".to_string());
    for tk in tokens {
        lines.push(match tk {
            Token::KeyWord(val) => format!("<keyword> {} </keyword>", val),
            Token::StringConstant(val) => format!("<stringConstant> {} </stringConstant>", val),
            Token::IntegerConstant(val) => format!("<integerConstant> {} </integerConstant>", val),
            Token::Symbol(val) => format!("<symbol> {} </symbol>", val),
            Token::Identifier(val) => format!("<identifier> {} </identifier>", val),
        })
    }
    lines.push("</tokens>".to_string());
    write_lines(&lines, file_path)
}
