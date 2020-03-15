use crate::constant::Token;
use crate::node::{Node, NodeType};

pub fn code_gen(ast: &Node) {
    let vm_codes = travel(ast, &String::from(""));

    println!("{}", vm_codes);
}

fn travel(node: &Node, cls: &str) -> String {
    match node.name {
        // class level
        NodeType::Class => {
            let class_name = node.children.get(1).unwrap();
            let mut vm_code: Vec<String> = vec![];
            if let Token::Identifier(cls) = &class_name.payload {
                for child in &node.children {
                    vm_code.push(travel(&child, cls))
                }
            }
            vm_code.join("")
        }

        // subroutineDec
        NodeType::SubRoutineDec => gen_subroutine_dec(node, cls),

        _ => String::new(),
    }
}

fn gen_subroutine_dec(node: &Node, cls: &str) -> String {
    let mut vm_code: Vec<String> = vec![];

    let function_name = node.children.get(2).unwrap();
    if let Token::Identifier(fn_name) = &function_name.payload {
        vm_code.push(format!("function {}.{}", cls, fn_name))
    }

    for child in &node.children {
        let child_code = match child.name {
            NodeType::ParameterList => gen_parameter_list(child),
            NodeType::SubRoutineBody => gen_subroutine_body(child),
            _ => String::new(),
        };
        vm_code.push(child_code);
    }

    vm_code.join("")
}

fn gen_parameter_list(node: &Node) -> String {
    for parameter in &node.children {
        println!("{:?}", parameter);
    }
    String::new()
}

fn gen_subroutine_body(node: &Node) -> String {
    let mut vm_code: Vec<String> = vec![];
    let statements = node.children.get(1).unwrap();

    for statement in &statements.children {
        let statement_code = match statement.name {
            NodeType::DoStatement => gen_do_statement(statement),
            _ => String::new(),
        };
        vm_code.push(statement_code);
    }
    vm_code.join("")
}

fn gen_do_statement(node: &Node) -> String {
    let mut vm_code: Vec<String> = vec![];

    let mut function_name = String::new();

    if let Token::Symbol(symbol) = &node.children.get(2).unwrap().payload {
        if symbol == "." {
            if let Token::Identifier(cls_name) = &node.children.get(1).unwrap().payload {
                if let Token::Identifier(fn_name) = &node.children.get(3).unwrap().payload {
                    function_name = format!("{}.{}", cls_name, fn_name)
                }
            }
        }
    }

    for do_statement_child in &node.children {
        if let NodeType::ExpressionList = do_statement_child.name {
            for expression in &do_statement_child.children {
                vm_code.push(gen_expression(expression))
            }
        }
    }

    vm_code.join("")
}

fn gen_expression(node: &Node) -> String {
    let mut vm_code: Vec<String> = vec![];

    for child in &node.children {
        match child.name {
            NodeType::Term => {
                println!("{:?}", child);
            }
            NodeType::Atom => {
                println!("{:?}", child);
            }
            _ => {}
        }
    }

    vm_code.join("")
}
