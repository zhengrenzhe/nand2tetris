use crate::instructions::*;
use crate::lexical::lexical;

pub fn code_gen(lines: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for (index, line) in lines.iter().enumerate() {
        if let Some(token) = lexical(line) {
            let codes = match token.command.as_str() {
                "push" => match token.target.as_str() {
                    "constant" => push_constant(token.arg),
                    "pointer" => push_pointer(token.arg),
                    _ => push(token.target, token.arg),
                },
                "pop" => match token.target.as_str() {
                    "pointer" => pop_pointer(token.arg),
                    _ => pop(token.target, token.arg),
                },
                "and" => and(),
                "or" => or(),
                "not" => not(),
                "neg" => neg(),
                "add" => add(),
                "sub" => sub(),
                "eq" => eq(index),
                "lt" => lt(index),
                "gt" => gt(index),
                "label" => label(token.target),
                "goto" => goto(token.target),
                "if-goto" => if_goto(token.target),
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

fn map_arg(target: &str) -> String {
    match target {
        "argument" => String::from("ARG"),
        "local" => String::from("LCL"),
        "this" => String::from("THIS"),
        "that" => String::from("THAT"),
        "temp" => String::from("TEMP"),
        "static" => String::from("STATIC"),
        _ => panic!(format!("target:{} not found", target)),
    }
}

fn push(target: String, arg: String) -> Vec<String> {
    let point_name = map_arg(&target);
    [
        vec![
            format!("// push {} {}", target, arg),
            switch_base_address(&point_name),
            switch_access_or_access_pointer(&point_name),
            format!("@{}", arg),
            String::from("A=D+A"),
            String::from("D=M"),
        ],
        push_d_to_stack(),
    ]
    .concat()
}

fn pop(target: String, arg: String) -> Vec<String> {
    let point_name = map_arg(&target);
    [
        vec![format!("// pop {} {}", target, arg)],
        dec_sp(),
        vec![
            switch_base_address(&point_name),
            switch_access_or_access_pointer(&point_name),
            format!("@{}", arg),
            String::from("D=D+A"),
            // save D
            String::from("@R13"),
            String::from("M=D"),
            // get value to save
            String::from("@SP"),
            String::from("AD=M"),
            String::from("D=M"),
            // save to M[R13]
            String::from("@R13"),
            String::from("A=M"),
            String::from("M=D"),
        ],
    ]
    .concat()
}

fn pop_pointer(arg: String) -> Vec<String> {
    [
        pop_stack_to_d(),
        vec![
            format!("@{}", if arg == "0" { "THIS" } else { "THAT" }),
            String::from("M=D"),
        ],
    ]
    .concat()
}

fn push_pointer(arg: String) -> Vec<String> {
    [
        vec![
            format!("@{}", if arg == "0" { "THIS" } else { "THAT" }),
            String::from("D=M"),
        ],
        push_d_to_stack(),
    ]
    .concat()
}

fn push_constant(arg: String) -> Vec<String> {
    [
        vec![
            format!("// push constant {}", arg),
            format!("@{}", arg),
            String::from("D=A"),
        ],
        push_d_to_stack(),
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
    ]
    .concat()
}

fn sub() -> Vec<String> {
    [
        vec![String::from("// sub")],
        pop_stack_to_d(),
        pop_stack_to_m(),
        vec![String::from("D=M-D")],
        push_d_to_stack(),
    ]
    .concat()
}

fn neg() -> Vec<String> {
    [
        vec![String::from("// neg")],
        pop_stack_to_d(),
        vec![String::from("D=-D")],
        push_d_to_stack(),
    ]
    .concat()
}

fn or() -> Vec<String> {
    [
        vec![String::from("// or")],
        pop_stack_to_d(),
        pop_stack_to_m(),
        vec![String::from("D=D|M")],
        push_d_to_stack(),
    ]
    .concat()
}

fn and() -> Vec<String> {
    [
        vec![String::from("// and")],
        pop_stack_to_d(),
        pop_stack_to_m(),
        vec![String::from("D=D&M")],
        push_d_to_stack(),
    ]
    .concat()
}

fn not() -> Vec<String> {
    [
        vec![String::from("// not")],
        pop_stack_to_d(),
        vec![String::from("D=!D")],
        push_d_to_stack(),
    ]
    .concat()
}

fn label(label_name: String) -> Vec<String> {
    vec![
        format!("// label {}", label_name),
        format!("({})", label_name),
    ]
}

fn goto(target: String) -> Vec<String> {
    vec![
        format!("// goto {}", target),
        format!("@{}", target),
        String::from("0;JEQ"),
    ]
}

fn if_goto(target: String) -> Vec<String> {
    [
        vec![format!("// if-goto {}", target)],
        pop_stack_to_d(),
        vec![format!("@{}", target), String::from("D;JNE")],
    ]
    .concat()
}
