pub fn match_token(word: &str) -> Token {
    if let Ok(val) = word.parse::<usize>() {
        return Token::IntegerConstant(val);
    }

    if word.starts_with('"') && word.ends_with('"') {
        return Token::StringConstant(word.trim_matches('"').to_string());
    }

    match word {
        "class" => Token::KeyWord("class".to_string()),
        "constructor" => Token::KeyWord("constructor".to_string()),
        "function" => Token::KeyWord("function".to_string()),
        "method" => Token::KeyWord("method".to_string()),
        "field" => Token::KeyWord("field".to_string()),
        "static" => Token::KeyWord("static".to_string()),
        "var" => Token::KeyWord("var".to_string()),
        "int" => Token::KeyWord("int".to_string()),
        "char" => Token::KeyWord("char".to_string()),
        "boolean" => Token::KeyWord("boolean".to_string()),
        "void" => Token::KeyWord("void".to_string()),
        "true" => Token::KeyWord("true".to_string()),
        "false" => Token::KeyWord("false".to_string()),
        "null" => Token::KeyWord("null".to_string()),
        "this" => Token::KeyWord("this".to_string()),
        "let" => Token::KeyWord("let".to_string()),
        "do" => Token::KeyWord("do".to_string()),
        "if" => Token::KeyWord("if".to_string()),
        "else" => Token::KeyWord("else".to_string()),
        "while" => Token::KeyWord("while".to_string()),
        "return" => Token::KeyWord("return".to_string()),
        "{" => Token::Symbol("{".to_string()),
        "}" => Token::Symbol("}".to_string()),
        "(" => Token::Symbol("(".to_string()),
        ")" => Token::Symbol(")".to_string()),
        "[" => Token::Symbol("[".to_string()),
        "]" => Token::Symbol("]".to_string()),
        "." => Token::Symbol(".".to_string()),
        "," => Token::Symbol(",".to_string()),
        ";" => Token::Symbol(";".to_string()),
        "+" => Token::Symbol("+".to_string()),
        "-" => Token::Symbol("-".to_string()),
        "*" => Token::Symbol("*".to_string()),
        "/" => Token::Symbol("/".to_string()),
        "&" => Token::Symbol("&amp;".to_string()),
        "|" => Token::Symbol("|".to_string()),
        "<" => Token::Symbol("&lt;".to_string()),
        ">" => Token::Symbol("&gt;".to_string()),
        "=" => Token::Symbol("=".to_string()),
        "~" => Token::Symbol("~".to_string()),
        _ => Token::Identifier(word.trim().trim_matches('"').to_string()),
    }
}

pub fn is_split_character(ch: char) -> bool {
    match ch {
        ' ' | '{' | '}' | '(' | ')' | '[' | ']' | '.' | ',' | ';' | '+' | '-' | '*' | '/' | '&'
        | '|' | '~' | '=' | '<' | '>' => true,
        _ => false,
    }
}

pub fn is_empty_character(ch: char) -> bool {
    match ch {
        '\n' => true,
        _ => false,
    }
}

#[derive(Clone, Debug)]
pub enum Token {
    KeyWord(String),
    Symbol(String),
    IntegerConstant(usize),
    StringConstant(String),
    Identifier(String),
    Empty,
}
