use std::collections::HashMap;

pub fn translate(lines: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for line in lines.iter() {
        if line.starts_with('@') {
            result.push(translate_a_instruction(line))
        } else {
            result.push(translate_c_instruction(line))
        }
    }

    result
}

fn translate_a_instruction(source: &str) -> String {
    let code = source.get(1..).expect("not found char after position 1");

    match code.parse::<u32>() {
        Ok(val) => {
            return format!("{:016b}", val);
        }
        Err(_) => println!("Err"),
    }

    code.to_string()
}

fn translate_c_instruction(source: &str) -> String {
    let c_struct = split_c_instruction(source);
    let dest_table = create_dest_table();
    let comp_table = create_comp_table();
    let jump_table = create_jump_table();

    let dest = *dest_table.get(c_struct.dest.as_str()).unwrap();
    let comp = *comp_table.get(c_struct.comp.as_str()).unwrap();
    let jump = *jump_table.get(c_struct.jump.as_str()).unwrap();

    format!("111{}{}{}", comp, dest, jump)
}

struct CInstruction {
    dest: String,
    comp: String,
    jump: String,
}

fn split_c_instruction(source: &str) -> CInstruction {
    let mut source_opt = source.to_string();

    let mut dest = String::from("");
    let mut jump = String::from("");

    // dest
    if let Some(equal_position) = source_opt.find('=') {
        dest = source_opt.get(0..equal_position).unwrap().to_string();
        source_opt = source_opt.replacen(dest.as_str(), "", 1);
        source_opt = source_opt.replace("=", "");
    }

    // jump
    if let Some(jump_position) = source_opt.find(';') {
        jump = source_opt.get((jump_position + 1)..).unwrap().to_string();
        source_opt = source_opt.replacen(jump.as_str(), "", 1);
        source_opt = source_opt.replace(";", "");
    }

    CInstruction {
        dest,
        comp: source_opt.clone(),
        jump,
    }
}

fn create_comp_table() -> HashMap<&'static str, &'static str> {
    let mut comp_table = HashMap::new();

    comp_table.insert("0", "0101010");
    comp_table.insert("1", "0111111");
    comp_table.insert("-1", "0111010");
    comp_table.insert("D", "0001100");
    comp_table.insert("A", "0110000");
    comp_table.insert("!D", "0001101");
    comp_table.insert("!A", "0110001");
    comp_table.insert("-D", "0001111");
    comp_table.insert("-A", "0110011");
    comp_table.insert("D+1", "0011111");
    comp_table.insert("A+1", "0110111");
    comp_table.insert("D-1", "0001110");
    comp_table.insert("A-1", "0110010");
    comp_table.insert("D+A", "0000010");
    comp_table.insert("D-A", "0010011");
    comp_table.insert("A-D", "0000111");
    comp_table.insert("D&A", "0000000");
    comp_table.insert("D|A", "0010101");
    comp_table.insert("M", "1110000");
    comp_table.insert("!M", "1110001");
    comp_table.insert("-M", "1110011");
    comp_table.insert("M+1", "1110111");
    comp_table.insert("M-1", "1110010");
    comp_table.insert("D+M", "1000010");
    comp_table.insert("D-M", "1010011");
    comp_table.insert("M-D", "1000111");
    comp_table.insert("D&M", "1000000");
    comp_table.insert("D|M", "1010101");

    comp_table
}

fn create_dest_table() -> HashMap<&'static str, &'static str> {
    let mut dest_table = HashMap::new();
    dest_table.insert("", "000");
    dest_table.insert("M", "001");
    dest_table.insert("D", "010");
    dest_table.insert("MD", "011");
    dest_table.insert("A", "100");
    dest_table.insert("AM", "101");
    dest_table.insert("AD", "110");
    dest_table.insert("AMD", "111");

    dest_table
}

fn create_jump_table() -> HashMap<&'static str, &'static str> {
    let mut jump_table = HashMap::new();
    jump_table.insert("", "000");
    jump_table.insert("JGT", "001");
    jump_table.insert("JEQ", "010");
    jump_table.insert("JGE", "011");
    jump_table.insert("JLT", "100");
    jump_table.insert("JNE", "101");
    jump_table.insert("JLE", "110");
    jump_table.insert("JMP", "111");

    jump_table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_a_instruction() {
        assert_eq!(translate_a_instruction("@2"), "0000000000000010");
        assert_eq!(translate_a_instruction("@3"), "0000000000000011");
        assert_eq!(translate_a_instruction("@0"), "0000000000000000");
    }

    #[test]
    fn test_split_c_instruction() {
        let a = split_c_instruction("D=A+M");
        assert_eq!(a.dest, "D");
        assert_eq!(a.comp, "A+M");
        assert_eq!(a.jump, "");

        let b = split_c_instruction("A+M");
        assert_eq!(b.dest, "");
        assert_eq!(b.comp, "A+M");
        assert_eq!(b.jump, "");

        let c = split_c_instruction("D=A;JMP");
        assert_eq!(c.dest, "D");
        assert_eq!(c.comp, "A");
        assert_eq!(c.jump, "JMP");

        let d = split_c_instruction("A+D;JMP");
        assert_eq!(d.dest, "");
        assert_eq!(d.comp, "A+D");
        assert_eq!(d.jump, "JMP");

        let e = split_c_instruction("0;JMP");
        assert_eq!(e.dest, "");
        assert_eq!(e.comp, "0");
        assert_eq!(e.jump, "JMP");

        let a = split_c_instruction("D=D+A");
        assert_eq!(a.dest, "D");
        assert_eq!(a.comp, "D+A");
        assert_eq!(a.jump, "");
    }

    #[test]
    fn test_translate_c_instruction() {
        assert_eq!(translate_c_instruction("D=A"), "1110110000010000");
        assert_eq!(translate_c_instruction("D=A;JEQ"), "1110110000010010");
        assert_eq!(translate_c_instruction("A;JEQ"), "1110110000000010");
        assert_eq!(translate_c_instruction("A"), "1110110000000000");
        assert_eq!(translate_c_instruction("M=D"), "1110001100001000");
        assert_eq!(translate_c_instruction("D=D+A"), "1110000010010000")
    }
}
