pub fn get_comp_binary(comp: &str) -> &str {
    if comp == "0" {
        return "0101010";
    }
    if comp == "1" {
        return "0111111";
    }
    if comp == "-1" {
        return "0111010";
    }
    if comp == "D" {
        return "0001100";
    }
    if comp == "A" {
        return "0110000";
    }
    if comp == "!D" {
        return "0001101";
    }
    if comp == "!A" {
        return "0110001";
    }
    if comp == "-D" {
        return "0001111";
    }
    if comp == "-A" {
        return "0110011";
    }
    if comp == "D+1" {
        return "0011111";
    }
    if comp == "A+1" {
        return "0110111";
    }
    if comp == "D-1" {
        return "0001110";
    }
    if comp == "A-1" {
        return "0110010";
    }
    if comp == "D+A" {
        return "0000010";
    }
    if comp == "D-A" {
        return "0010011";
    }
    if comp == "A-D" {
        return "0000111";
    }
    if comp == "D&A" {
        return "0000000";
    }
    if comp == "D|A" {
        return "0010101";
    }
    if comp == "M" {
        return "1110000";
    }
    if comp == "!M" {
        return "1110001";
    }
    if comp == "-M" {
        return "1110011";
    }
    if comp == "M+1" {
        return "1110111";
    }
    if comp == "M-1" {
        return "1110010";
    }
    if comp == "D+M" {
        return "1000010";
    }
    if comp == "D-M" {
        return "1010011";
    }
    if comp == "M-D" {
        return "1000111";
    }
    if comp == "D&M" {
        return "1000000";
    }
    if comp == "D|M" {
        return "1010101";
    }
    ""
}

pub fn get_dest_binary(dest: &str) -> &str {
    if dest == "" {
        return "000";
    }
    if dest == "M" {
        return "001";
    }
    if dest == "D" {
        return "010";
    }
    if dest == "MD" {
        return "011";
    }
    if dest == "A" {
        return "100";
    }
    if dest == "AM" {
        return "101";
    }
    if dest == "AD" {
        return "110";
    }
    if dest == "AMD" {
        return "111";
    }
    ""
}

pub fn get_jump_binary(jump: &str) -> &str {
    if jump == "" {
        return "000";
    }
    if jump == "JGT" {
        return "001";
    }
    if jump == "JEQ" {
        return "010";
    }
    if jump == "JGE" {
        return "011";
    }
    if jump == "JLT" {
        return "100";
    }
    if jump == "JNE" {
        return "101";
    }
    if jump == "JLE" {
        return "110";
    }
    if jump == "JMP" {
        return "111";
    }
    ""
}
