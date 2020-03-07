use crate::constant::Token;
use crate::index::Index;
use crate::node::{Node, NodeType};

pub fn parser(tokens: &[Token]) -> Node {
    compile_class(tokens)
}

fn compile_class(tokens: &[Token]) -> Node {
    let mut class_node = Node::new(NodeType::Class, Token::Empty);
    let mut index = Index::new(0, tokens.len());

    // class
    class_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
    // class name
    class_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
    // {
    class_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

    while index.has_next() {
        if let Token::KeyWord(tk) = tokens[index.preview()].clone() {
            // variable
            if tk == "static" || tk == "field" {
                class_node.append_child(compile_class_var_dec(tokens, &mut index));
            } else if tk == "constructor" || tk == "function" || tk == "method" {
                class_node.append_child(compile_subroutine(tokens, &mut index));
            }
        }
    }

    class_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

    class_node
}

fn check_symbol(tokens: &[Token], index: &Index, target: &str) -> bool {
    if let Token::Symbol(val) = tokens[index.preview()].clone() {
        if val == target {
            return true;
        }
    }
    false
}

// syntax static int foo, bar;
fn compile_class_var_dec(tokens: &[Token], index: &mut Index) -> Node {
    let mut class_var_node = Node::new(NodeType::ClassVarDec, Token::Empty);

    // (static | field)
    class_var_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
    // (type)
    class_var_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
    // (varName)
    class_var_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

    loop {
        if check_symbol(tokens, &index, ";") {
            break;
        }
        class_var_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
    }

    // ;
    class_var_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

    class_var_node
}

fn compile_subroutine(tokens: &[Token], index: &mut Index) -> Node {
    let mut subroutine_node = Node::new(NodeType::SubRoutineDec, Token::Empty);

    // (constructor | function | method)
    subroutine_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
    // return type
    subroutine_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
    // subroutine name
    subroutine_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
    // (
    subroutine_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

    // parameter list
    if check_symbol(tokens, index, ")") {
        subroutine_node.append_child(compile_parameter_list(tokens, index, true));
    } else {
        subroutine_node.append_child(compile_parameter_list(tokens, index, false));
    }

    // )
    subroutine_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

    subroutine_node.append_child(compile_subroutine_body(tokens, index));

    subroutine_node
}

fn compile_parameter_list(tokens: &[Token], index: &mut Index, empty: bool) -> Node {
    let mut parameter_list_node = Node::new(NodeType::ParameterList, Token::Empty);

    if empty {
        return parameter_list_node;
    }

    loop {
        if check_symbol(tokens, index, ")") {
            break;
        }
        parameter_list_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
    }

    parameter_list_node
}

fn compile_subroutine_body(tokens: &[Token], index: &mut Index) -> Node {
    let mut subroutine_body_node = Node::new(NodeType::SubRoutineBody, Token::Empty);

    // {
    subroutine_body_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

    loop {
        match tokens[index.preview()].clone() {
            Token::KeyWord(val) => {
                if val == "var" {
                    subroutine_body_node.append_child(compile_var_dec(tokens, index));
                } else {
                    subroutine_body_node.append_child(compile_statements(tokens, index));
                }
            }
            Token::Symbol(val) => {
                if val == "}" {
                    break;
                } else {
                    subroutine_body_node.append_child(compile_statements(tokens, index));
                }
            }
            _ => {
                subroutine_body_node.append_child(compile_statements(tokens, index));
            }
        }
    }

    //}
    subroutine_body_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

    subroutine_body_node
}

fn compile_var_dec(tokens: &[Token], index: &mut Index) -> Node {
    let mut var_dec_node = Node::new(NodeType::VarDec, Token::Empty);

    loop {
        var_dec_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

        if let Token::Symbol(val) = tokens[index.preview()].clone() {
            if val == ";" {
                var_dec_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
                return var_dec_node;
            }
        }
    }
}

fn compile_statements(tokens: &[Token], index: &mut Index) -> Node {
    let mut statements_node = Node::new(NodeType::Statements, Token::Empty);

    loop {
        if check_symbol(tokens, index, "}") {
            break;
        }
        if let Token::KeyWord(val) = tokens[index.preview()].clone() {
            match val.as_str() {
                "let" => statements_node.append_child(compile_let(tokens, index)),
                "if" => statements_node.append_child(compile_if(tokens, index)),
                "while" => statements_node.append_child(compile_while(tokens, index)),
                "do" => statements_node.append_child(compile_do(tokens, index)),
                "return" => statements_node.append_child(compile_return(tokens, index)),
                _ => {}
            }
        }
    }
    statements_node
}

fn compile_let(tokens: &[Token], index: &mut Index) -> Node {
    let mut let_node = Node::new(NodeType::LetStatement, Token::Empty);

    // let
    let_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
    // variable name
    let_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

    // []
    if let Token::Symbol(val) = tokens[index.preview()].clone() {
        if val == "[" {
            let_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
            let_node.append_child(compile_expression(tokens, index, "]", ""));
            let_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
        }
    }

    // =
    let_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
    // expression
    let_node.append_child(compile_expression(tokens, index, ";", ""));
    // ;
    let_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

    let_node
}

fn compile_expression(
    tokens: &[Token],
    index: &mut Index,
    end_char: &str,
    end_char2: &str,
) -> Node {
    let mut expression_node = Node::new(NodeType::Expression, Token::Empty);

    loop {
        if let Token::Symbol(val) = tokens[index.preview()].clone() {
            if val == end_char {
                return expression_node;
            }
            if val == end_char2 {
                return expression_node;
            }
            if val == "-" || val == "~" {
                if let Token::Symbol(val_prev) = tokens[index.prev()].clone() {
                    match val_prev.as_str() {
                        "+" | "-" | "*" | "/" | "&amp;" | "|" | "&lt;" | "&gt;" | "=" | "(" => {
                            expression_node.append_child(compile_term(tokens, index));
                            continue;
                        }
                        _ => {}
                    }
                }
            }
            match val.as_str() {
                "+" | "-" | "*" | "/" | "&amp;" | "|" | "&lt;" | "&gt;" | "=" => expression_node
                    .append_child(Node::new(NodeType::Atom, tokens[index.get()].clone())),
                _ => {
                    expression_node.append_child(compile_term(tokens, index));
                }
            }
        } else {
            expression_node.append_child(compile_term(tokens, index));
        }
    }
}

fn compile_term(tokens: &[Token], index: &mut Index) -> Node {
    let mut term_node = Node::new(NodeType::Term, Token::Empty);

    match tokens[index.preview()].clone() {
        Token::IntegerConstant(_) | Token::StringConstant(_) => {
            term_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
            return term_node;
        }
        Token::KeyWord(_) => {
            term_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
            return term_node;
        }
        Token::Symbol(val) => {
            if val == "(" {
                term_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
                term_node.append_child(compile_expression(tokens, index, ")", ""));
                term_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
                return term_node;
            }
            if val == "-" || val == "~" {
                term_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
                term_node.append_child(compile_term(tokens, index));
                return term_node;
            }
        }
        Token::Identifier(_) => {
            term_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
            if let Token::Symbol(val) = tokens[index.preview()].clone() {
                if val == "[" {
                    term_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
                    term_node.append_child(compile_expression(tokens, index, "]", ""));
                    term_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
                    return term_node;
                }
                if val == "." || val == "(" {
                    for n in compile_subroutine_call(tokens, index) {
                        term_node.append_child(n);
                    }
                }
            }
        }
        _ => {}
    }

    term_node
}

fn compile_subroutine_call(tokens: &[Token], index: &mut Index) -> Vec<Node> {
    let mut result: Vec<Node> = vec![];

    if let Token::Symbol(val) = tokens[index.preview()].clone() {
        if val == "." {
            result.push(Node::new(NodeType::Atom, tokens[index.get()].clone()));
            result.push(Node::new(NodeType::Atom, tokens[index.get()].clone()));
        }
        if let Token::Symbol(val) = tokens[index.preview()].clone() {
            if val == "(" {
                // (
                result.push(Node::new(NodeType::Atom, tokens[index.get()].clone()));
                // parameters
                result.push(compile_expression_list(tokens, index));
                // )
                result.push(Node::new(NodeType::Atom, tokens[index.get()].clone()));
            }
        }
    }

    result
}

fn compile_expression_list(tokens: &[Token], index: &mut Index) -> Node {
    let mut expression_list_node = Node::new(NodeType::ExpressionList, Token::Empty);

    loop {
        if let Token::Symbol(val) = tokens[index.preview()].clone() {
            if val == ")" {
                return expression_list_node;
            }
            if val == "," {
                expression_list_node
                    .append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
            }
        }
        expression_list_node.append_child(compile_expression(tokens, index, ",", ")"));
    }
}

fn compile_if(tokens: &[Token], index: &mut Index) -> Node {
    let mut if_node = Node::new(NodeType::IfStatement, Token::Empty);

    // if
    if_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
    // (
    if_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
    // expression
    if_node.append_child(compile_expression(tokens, index, ")", ""));
    // )
    if_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
    // {
    if_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
    // statements
    if_node.append_child(compile_statements(tokens, index));
    // }
    if_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

    // else block
    if let Token::KeyWord(val) = tokens[index.preview()].clone() {
        if val == "else" {
            if_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
            if_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
            if_node.append_child(compile_statements(tokens, index));
            if_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
        }
    }

    if_node
}

fn compile_while(tokens: &[Token], index: &mut Index) -> Node {
    let mut while_node = Node::new(NodeType::WhileStatement, Token::Empty);

    // while
    while_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

    // (
    while_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

    // expression
    while_node.append_child(compile_expression(tokens, index, ")", ""));

    // )
    while_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

    // {
    while_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

    // statements
    while_node.append_child(compile_statements(tokens, index));

    // }
    while_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

    while_node
}

fn compile_do(tokens: &[Token], index: &mut Index) -> Node {
    let mut do_node = Node::new(NodeType::DoStatement, Token::Empty);

    // do
    do_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));
    // function name
    do_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

    for n in compile_subroutine_call(tokens, index) {
        do_node.append_child(n);
    }

    do_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

    do_node
}

fn compile_return(tokens: &[Token], index: &mut Index) -> Node {
    let mut return_node = Node::new(NodeType::ReturnStatement, Token::Empty);

    // return
    return_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

    // expression
    if !check_symbol(tokens, index, ";") {
        return_node.append_child(compile_expression(tokens, index, ";", ""));
    }

    // ;
    return_node.append_child(Node::new(NodeType::Atom, tokens[index.get()].clone()));

    return_node
}
