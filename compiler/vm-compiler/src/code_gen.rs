use crate::instructions::*;
use crate::lexical::lexical;

pub fn code_gen(lines: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for (index, line) in lines.iter().enumerate() {
        if let Some(token) = lexical(line) {
            let codes = match token.command.as_str() {
                "push" => match token.target.as_str() {
                    "constant" => push_constant(token.arg),
                    _ => vec![],
                },
                "add" => add(),
                "eq" => eq(index),
                "lt" => lt(index),
                "gt" => gt(index),
                _ => vec![],
            };
            for code in codes {
                result.push(code);
            }
        }
        result.push(String::from("\n"));
    }

    result
}

fn push_constant(arg: String) -> Vec<String> {
    [
        vec![
            format!("// push constant {}", arg),
            format!("@{}", arg),
            String::from("D=A"),
        ],
        push_d_to_stack(),
        sp_plus_plus(),
    ]
    .concat()
}

fn add() -> Vec<String> {
    [
        vec![String::from("// add")],
        pop_stack_to_d(),
        pop_stack_to_m(),
        vec![String::from("D=D+M")],
        push_d_to_stack(),
        sp_plus_plus(),
    ]
    .concat()
}

fn eq(id: usize) -> Vec<String> {
    [
        vec![String::from("// eq")],
        pop_stack_to_d(),
        pop_stack_to_m(),
        // eq
        vec![String::from("D=D-M")],
        if_then_else(
            id,
            String::from("D;JEQ"),
            String::from("D;JNE"),
            String::from("D=-1"),
            String::from("D=0"),
        ),
        push_d_to_stack(),
        sp_plus_plus(),
    ]
    .concat()
}

fn lt(id: usize) -> Vec<String> {
    [
        vec![String::from("// lt")],
        pop_stack_to_d(),
        pop_stack_to_m(),
        // lt
        vec![String::from("D=M-D")],
        if_then_else(
            id,
            String::from("D;JLT"),
            String::from("D;JGE"),
            String::from("D=-1"),
            String::from("D=0"),
        ),
        push_d_to_stack(),
        sp_plus_plus(),
    ]
    .concat()
}

fn gt(id: usize) -> Vec<String> {
    [
        vec![String::from("// lt")],
        pop_stack_to_d(),
        pop_stack_to_m(),
        // gt
        vec![String::from("D=M-D")],
        if_then_else(
            id,
            String::from("D;JGT"),
            String::from("D;JLE"),
            String::from("D=-1"),
            String::from("D=0"),
        ),
        push_d_to_stack(),
        sp_plus_plus(),
    ]
    .concat()
}
