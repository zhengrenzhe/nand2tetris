use crate::instructions::*;
use crate::lexical::lexical;

pub fn code_gen(mut lines: Vec<String>, is_insert_bootstrap: bool) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    if is_insert_bootstrap {
        lines.insert(0, String::from("call Sys.init 0"));
    }

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
                "call" => call(token.target, token.arg),
                "function" => function(token.target, token.arg),
                "return" => ret(),
                _ => vec![],
            };
            for code in codes {
                result.push(code);
            }
        }
        result.push(String::from("\n"));
    }

    if is_insert_bootstrap {
        result.insert(0, String::from("M=D"));
        result.insert(0, String::from("@SP"));
        result.insert(0, String::from("D=A"));
        result.insert(0, String::from("@256"));
    }

    result
}

fn call(target: String, arg: String) -> Vec<String> {
    [
        // push return address to stack
        vec![
            format!("// call {} {}", target, arg),
            format!("@{}callerReturnAddress", target),
            String::from("D=A"),
        ],
        push_d_to_stack(),
        // save frame
        save_frame("LCL"),
        save_frame("ARG"),
        save_frame("THIS"),
        save_frame("THAT"),
        // set ARG to position of the first argument(SP-5-arg)
        vec![
            String::from("@SP"),
            String::from("D=M"),
            String::from("@5"),
            String::from("D=D-A"),
            format!("@{}", arg),
            String::from("D=D-A"),
            String::from("@ARG"),
            String::from("M=D"),
        ],
        // set LCL to SP
        vec![
            String::from("@SP"),
            String::from("D=M"),
            String::from("@LCL"),
            String::from("M=D"),
        ],
        // goto function
        vec![format!("@{}", target), String::from("0;JEQ")],
        // set return address label
        vec![format!("({}callerReturnAddress)", target)],
    ]
    .concat()
}

fn save_frame(target: &str) -> Vec<String> {
    [
        vec![format!("@{}", target), String::from("D=M")],
        push_d_to_stack(),
    ]
    .concat()
}

fn function(target: String, arg: String) -> Vec<String> {
    let arg_count: usize = arg.parse().unwrap();
    [
        vec![
            format!("// function {} {}", target, arg),
            format!("({})", target),
        ],
        // push n args to stack
        (0..arg_count)
            .enumerate()
            .map(|_| {
                [
                    vec![String::from("@0"), String::from("D=A")],
                    push_d_to_stack(),
                ]
                .concat()
            })
            .collect::<Vec<Vec<String>>>()
            .concat(),
        // set LCL
        vec![
            // save arg to R13
            format!("@{}", arg),
            String::from("D=A"),
            String::from("@R13"),
            String::from("M=D"),
            // LCL = SP - arg
            String::from("@SP"),
            String::from("D=M"),
            String::from("@R13"),
            String::from("D=D-M"),
            String::from("@LCL"),
            String::from("M=D"),
        ],
    ]
    .concat()
}

fn ret() -> Vec<String> {
    [
        vec![
            String::from("// return"),
            // endFrame = LCL
            String::from("@LCL"),
            String::from("D=M"),
            String::from("@endFrame"),
            String::from("M=D"),
            // retAddr = *(endFrame-5)
            String::from("@5"),
            String::from("D=D-A"),
            String::from("A=D"),
            String::from("D=M"),
            String::from("@retAddr"),
            String::from("M=D"),
        ],
        // pop value to *ARG
        pop_stack_to_d(),
        vec![
            String::from("@ARG"),
            String::from("A=M"),
            String::from("M=D"),
        ],
        // SP=ARG+1
        vec![
            String::from("D=A+1"),
            String::from("@SP"),
            String::from("M=D"),
        ],
        // restore THAT, THIS, ARG, LCL
        restore_frame("THAT", "1"),
        restore_frame("THIS", "2"),
        restore_frame("ARG", "3"),
        restore_frame("LCL", "4"),
        vec![
            String::from("@retAddr"),
            String::from("A=M"),
            String::from("0;JEQ"),
        ],
    ]
    .concat()
}

fn restore_frame(target: &str, diff: &str) -> Vec<String> {
    vec![
        format!("@{}", diff),
        String::from("D=A"),
        String::from("@endFrame"),
        String::from("A=M-D"),
        String::from("D=M"),
        format!("@{}", target),
        String::from("M=D"),
    ]
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
