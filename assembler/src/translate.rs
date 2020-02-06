use crate::static_table::{get_comp_binary, get_dest_binary, get_jump_binary};

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

    let dest = get_dest_binary(c_struct.dest.as_str());
    let comp = get_comp_binary(c_struct.comp.as_str());
    let jump = get_jump_binary(c_struct.jump.as_str());

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
