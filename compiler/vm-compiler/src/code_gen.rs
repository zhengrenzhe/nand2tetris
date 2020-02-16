use crate::lexical::lexical;

pub fn code_gen(lines: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for line in lines {
        if let Some(token) = lexical(&line) {
            let codes = match token.command.as_str() {
                "push" => match token.target.as_str() {
                    "constant" => push_constant(token.arg),
                    _ => vec![],
                },
                "add" => add(),
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
        // *sp = i
        format!("@{}", arg),
        String::from("D=A"),
        String::from("@SP"),
        String::from("M=D"),
        // sp += 1
        String::from("A=A+1"),
    ]
}

fn add() -> Vec<String> {
    vec![
        String::from("// add"),
        String::from("@SP"),
        String::from("A=A-1"),
        String::from("D=M"),
        String::from("A=A-1"),
        String::from("D=D+M"),
        String::from("M=D"),
        String::from("A=A+1"),
    ]
}
