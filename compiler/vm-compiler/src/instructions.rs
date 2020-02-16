pub fn dec_sp() -> Vec<String> {
    vec![String::from("@SP"), String::from("M=M-1")]
}

pub fn push_d_to_stack() -> Vec<String> {
    vec![
        String::from("@SP"),
        String::from("A=M"),
        String::from("M=D"),
        String::from("@SP"),
        String::from("M=M+1"),
    ]
}

pub fn pop_stack_to_d() -> Vec<String> {
    vec![
        String::from("@SP"),
        String::from("M=M-1"),
        String::from("A=M"),
        String::from("D=M"),
    ]
}

pub fn pop_stack_to_m() -> Vec<String> {
    vec![
        String::from("@SP"),
        String::from("M=M-1"),
        String::from("A=M"),
    ]
}

pub fn use_label(id: usize, name: &str) -> String {
    format!("@label{}_{}", id, name)
}

pub fn label(id: usize, name: &str) -> String {
    format!("(label{}_{})", id, name)
}

pub fn if_then_else(
    id: usize,
    true_condition: String,
    false_condition: String,
    then_block: String,
    false_block: String,
) -> Vec<String> {
    vec![
        use_label(id, "true_block"),
        true_condition,
        //
        use_label(id, "false_block"),
        false_condition,
        //
        use_label(id, "pass"),
        String::from("0;JEQ"),
        //
        label(id, "true_block"),
        then_block,
        use_label(id, "pass"),
        String::from("0;JEQ"),
        //
        label(id, "false_block"),
        false_block,
        //
        label(id, "pass"),
    ]
}
