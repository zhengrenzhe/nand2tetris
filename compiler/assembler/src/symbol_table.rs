use std::collections::HashMap;

fn get_system_binary(code: &str) -> &'static str {
    match code {
        "SP" => "0000000000000000",
        "LCL" => "0000000000000001",
        "ARG" => "0000000000000010",
        "THIS" => "0000000000000011",
        "THAT" => "0000000000000100",
        "SCREEN" => "0100000000000000",
        "KBD" => "0110000000000000",
        "R0" => "0000000000000000",
        "R1" => "0000000000000001",
        "R2" => "0000000000000010",
        "R3" => "0000000000000011",
        "R4" => "0000000000000100",
        "R5" => "0000000000000101",
        "R6" => "0000000000000110",
        "R7" => "0000000000000111",
        "R8" => "0000000000001000",
        "R9" => "0000000000001001",
        "R10" => "0000000000001010",
        "R11" => "0000000000001011",
        "R12" => "0000000000001100",
        "R13" => "0000000000001101",
        "R14" => "0000000000001110",
        "R15" => "0000000000001111",
        _ => "_",
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: HashMap<String, String>,
    base_address: i32,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbols: HashMap::new(),
            base_address: 16,
        }
    }

    pub fn add(&mut self, symbol: String, value: String) {
        self.symbols.insert(symbol, value);
    }

    pub fn get(&mut self, symbol: &str) -> String {
        let system_symbol = get_system_binary(symbol);
        if system_symbol != "_" {
            return String::from(system_symbol);
        }

        if let Some(val) = self.symbols.get(symbol) {
            return String::from(val);
        }

        let new_address = format!("0{:015b}", self.base_address);

        self.base_address += 1;

        self.add(String::from(symbol), new_address.clone());

        new_address
    }
}

pub fn generate_symbol_table(mut lines: Vec<String>) -> (Vec<String>, SymbolTable) {
    let mut symbol_table = SymbolTable::new();
    let mut label_indexes: Vec<usize> = vec![];
    let mut line_number = 0;

    for line in lines.iter() {
        if line.starts_with('(') {
            label_indexes.push(line_number);
            continue;
        }
        line_number += 1;
    }

    for label_index in label_indexes {
        let label = lines.remove(label_index).replace("(", "").replace(")", "");
        // convert label line number to binary format
        // line number start from 0
        symbol_table.add(label, format!("0{:015b}", label_index))
    }

    (lines, symbol_table)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_symbol_table() {
        let (codes, mut symbol_table) = generate_symbol_table(vec![
            String::from("AA"),
            String::from("(LOOP)"),
            String::from("BB"),
            String::from("(GB)"),
            String::from("CC"),
            String::from("DD"),
            String::from("(END)"),
            String::from("EE"),
        ]);

        assert_eq!(codes.len(), 5);
        assert_eq!(symbol_table.symbols.len(), 3);

        assert_eq!(symbol_table.get("LOOP"), format!("0{:015b}", 1));
        assert_eq!(symbol_table.get("GB"), format!("0{:015b}", 2));
        assert_eq!(symbol_table.get("END"), format!("0{:015b}", 4));
        assert_eq!(symbol_table.get("LA"), format!("0{:015b}", 16));
        assert_eq!(symbol_table.get("LB"), format!("0{:015b}", 17));
        assert_eq!(symbol_table.get("LC"), format!("0{:015b}", 18));
    }
}
