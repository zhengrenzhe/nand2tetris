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

// push constant 12
// push number 12 to stack.
fn push_constant(arg: String) -> Vec<String> {
    vec![
        format!("// push constant {}", arg),
        format!("@{}", arg),
        String::from("D=A"),
        // push value stack
        String::from("@SP"),
        String::from("A=M"),
        String::from("M=D"),
        // sp += 1
        String::from("@SP"),
        String::from("M=M+1"),
    ]
}

fn add() -> Vec<String> {
    vec![
        String::from("// add"),
        // get first value
        String::from("@SP"),   // A=0
        String::from("M=M-1"), // M[0] = M[0] -1, *sp = 257
        String::from("A=M"),   // A=257
        String::from("D=M"),   // D=M[257] = 8
        // get second value
        String::from("@SP"),   // a=0
        String::from("M=M-1"), // M[0] = M[0] -1, *sp = 256
        String::from("A=M"),   // A=256
        // add and set result
        String::from("D=D+M"), // D = 8+M[256] = 8+7 = 15
        String::from("@SP"),   // A = 0
        String::from("A=M"),   // A = M[0] = 256
        String::from("M=D"),   // M[256] = D = 15
        // sp += 1
        String::from("@SP"),   // A=0
        String::from("M=M+1"), // M[0] = M[0] +1, *sp = 257
    ]
}

fn eq(id: usize) -> Vec<String> {
    vec![
        String::from("// eq"),
        // get first value
        String::from("@SP"),   // A=0
        String::from("M=M-1"), // M[0] = M[0] -1, *sp = 257
        String::from("A=M"),   // A=257
        String::from("D=M"),   // D=M[257] = 8
        // get second value
        String::from("@SP"),   // a=0
        String::from("M=M-1"), // M[0] = M[0] -1, *sp = 256
        String::from("A=M"),   // A=256
        // eq
        String::from("D=D-M"),
        format!("@label{}dIsZero", id),
        String::from("D;JEQ"),
        format!("@label{}dIsNotZero", id),
        String::from("D;JNE"),
        format!("(label{}dIsZero)", id),
        String::from("D=-1"),
        format!("@label{}end", id),
        String::from("0;JEQ"),
        format!("(label{}dIsNotZero)", id),
        String::from("D=0"),
        format!("(label{}end)", id),
        // save
        String::from("@SP"), // A = 0
        String::from("A=M"), // A = M[0] = 256
        String::from("M=D"), // M[256] = D = 15
        // sp += 1
        String::from("@SP"),   // A=0
        String::from("M=M+1"), // M[0] = M[0] +1, *sp = 257
    ]
}
