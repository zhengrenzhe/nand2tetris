use std::io::Error;

use crate::constant::Token;
use crate::node::{Node, NodeType};
use utils::io::{write_lines, write_string};

pub fn write_tokens_xml(tokens: &[Token], file_path: &str) -> Result<bool, Error> {
    let mut lines: Vec<String> = vec![];
    lines.push("<tokens>".to_string());
    for tk in tokens {
        lines.push(gen_tag(tk, 0))
    }
    lines.push("</tokens>".to_string());
    write_lines(&lines, file_path)
}

pub fn write_ast_xml(node: Node, file_path: &str) -> Result<bool, Error> {
    // println!("{:#?}", node);
    write_string(travel_tree(&node, 0), file_path)?;
    Ok(true)
}

fn travel_tree(node: &Node, indent: i32) -> String {
    let mut result: Vec<String> = vec![];
    let tag_name = match node.name {
        NodeType::Class => "class",
        NodeType::ClassVarDec => "classVarDec",
        NodeType::ReturnStatement => "returnStatement",
        NodeType::DoStatement => "doStatement",
        NodeType::WhileStatement => "whileStatement",
        NodeType::IfStatement => "ifStatement",
        NodeType::ExpressionList => "expressionList",
        NodeType::Term => "term",
        NodeType::LetStatement => "letStatement",
        NodeType::Statements => "statements",
        NodeType::VarDec => "varDec",
        NodeType::SubRoutineBody => "subroutineBody",
        NodeType::ParameterList => "parameterList",
        NodeType::SubRoutineDec => "subroutineDec",
        NodeType::Expression => "expression",
        NodeType::Atom => "atom",
    };

    result.push(format!("{}<{}>\n", left_pad(indent), tag_name));
    for child in node.clone().children {
        match child.name {
            NodeType::Atom => result.push(gen_tag(&child.payload, indent + 2)),
            _ => result.push(travel_tree(&child, indent + 2)),
        }
    }
    result.push(format!("{}</{}>\n", left_pad(indent), tag_name));
    result.join("")
}

fn gen_tag(token: &Token, indent: i32) -> String {
    let indent_pad = left_pad(indent);
    match token {
        Token::KeyWord(val) => format!("{}<keyword> {} </keyword>\n", indent_pad, val),
        Token::StringConstant(val) => {
            format!("{}<stringConstant> {} </stringConstant>\n", indent_pad, val)
        }
        Token::IntegerConstant(val) => format!(
            "{}<integerConstant> {} </integerConstant>\n",
            indent_pad, val
        ),
        Token::Symbol(val) => format!("{}<symbol> {} </symbol>\n", indent_pad, val),
        Token::Identifier(val) => format!("{}<identifier> {} </identifier>\n", indent_pad, val),
        _ => String::new(),
    }
}

fn left_pad(indent: i32) -> String {
    let mut indent_pad: Vec<&str> = vec![];
    for _ in 0..indent {
        indent_pad.push(" ");
    }
    indent_pad.join("")
}
