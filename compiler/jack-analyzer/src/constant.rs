pub fn match_token(word: &str) -> Token {
    if let Ok(val) = word.parse::<usize>() {
        return Token::IntegerConstant(val);
    }

    if word.starts_with('"') && word.ends_with('"') {
        return Token::StringConstant(word.trim_matches('"'));
    }

    match word {
        "class" => Token::KeyWord("class"),
        "constructor" => Token::KeyWord("constructor"),
        "function" => Token::KeyWord("function"),
        "method" => Token::KeyWord("method"),
        "field" => Token::KeyWord("field"),
        "static" => Token::KeyWord("static"),
        "var" => Token::KeyWord("var"),
        "int" => Token::KeyWord("int"),
        "char" => Token::KeyWord("char"),
        "boolean" => Token::KeyWord("boolean"),
        "void" => Token::KeyWord("void"),
        "true" => Token::KeyWord("true"),
        "false" => Token::KeyWord("false"),
        "null" => Token::KeyWord("null"),
        "this" => Token::KeyWord("this"),
        "let" => Token::KeyWord("let"),
        "do" => Token::KeyWord("do"),
        "if" => Token::KeyWord("if"),
        "else" => Token::KeyWord("else"),
        "while" => Token::KeyWord("while"),
        "return" => Token::KeyWord("return"),
        "{" => Token::Symbol("{"),
        "}" => Token::Symbol("}"),
        "(" => Token::Symbol("("),
        ")" => Token::Symbol(")"),
        "[" => Token::Symbol("["),
        "]" => Token::Symbol("]"),
        "." => Token::Symbol("."),
        "," => Token::Symbol(","),
        ";" => Token::Symbol(";"),
        "+" => Token::Symbol("+"),
        "-" => Token::Symbol("-"),
        "*" => Token::Symbol("*"),
        "/" => Token::Symbol("/"),
        "&" => Token::Symbol("&"),
        "|" => Token::Symbol("|"),
        "<" => Token::Symbol("<"),
        ">" => Token::Symbol(">"),
        "=" => Token::Symbol("="),
        "~" => Token::Symbol("~"),
        _ => Token::Identifier(word),
    }
}

pub enum Token<'a> {
    KeyWord(&'a str),
    Symbol(&'a str),
    IntegerConstant(usize),
    StringConstant(&'a str),
    Identifier(&'a str),
}
