#[derive(Debug)]
pub struct Token {
    command: String,
    args: Vec<String>,
}

pub fn lexical(code: &str) -> Option<Token> {
    let parts: Vec<&str> = code.split(' ').filter(|code| !code.is_empty()).collect();

    if parts.is_empty() {
        return None;
    }

    Some(Token {
        command: String::from(parts[0]),
        args: parts[1..].iter().map(|item| String::from(*item)).collect(),
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

        match lexical("push") {
            Some(token) => {
                assert_eq!(token.command, String::from("push"));
                assert_eq!(token.args.len(), 0);
            }
            None => panic!("should not be None"),
        }

        match lexical("push 12") {
            Some(token) => {
                assert_eq!(token.command, String::from("push"));
                assert_eq!(token.args.len(), 1);
                assert_eq!(token.args[0], String::from("12"));
            }
            None => panic!("should not be None"),
        }

        match lexical("push constant 12") {
            Some(token) => {
                assert_eq!(token.command, String::from("push"));
                assert_eq!(token.args.len(), 2);
                assert_eq!(token.args[0], String::from("constant"));
                assert_eq!(token.args[1], String::from("12"));
            }
            None => panic!("should not be None"),
        }
    }
}
