#[derive(Debug)]
pub struct Token {
    pub command: String,
    pub target: String,
    pub arg: String,
}

pub fn lexical(code: &str) -> Option<Token> {
    let parts: Vec<&str> = code.split(' ').filter(|code| !code.is_empty()).collect();

    if parts.is_empty() {
        return None;
    }

    Some(Token {
        command: String::from(parts[0]),
        target: String::from(*parts.get(1).unwrap_or(&"")),
        arg: String::from(*parts.get(2).unwrap_or(&"")),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexical_analysis() {
        if let Some(val) = lexical("") {
            panic!(format!("val: {:?} should be None", val));
        }

        match lexical("add") {
            Some(token) => {
                assert_eq!(token.command, String::from("add"));
                assert_eq!(token.target, String::from(""));
                assert_eq!(token.arg, String::from(""));
            }
            None => panic!("should not be None"),
        }

        match lexical("push 12") {
            Some(token) => {
                assert_eq!(token.command, String::from("push"));
                assert_eq!(token.target, String::from("12"));
                assert_eq!(token.arg, String::from(""));
            }
            None => panic!("should not be None"),
        }

        match lexical("push constant 12") {
            Some(token) => {
                assert_eq!(token.command, String::from("push"));
                assert_eq!(token.target, String::from("constant"));
                assert_eq!(token.arg, String::from("12"));
            }
            None => panic!("should not be None"),
        }
    }
}
