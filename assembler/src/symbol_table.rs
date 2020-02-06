use std::collections::HashMap;

use crate::static_table::get_system_binary;

pub struct SymbolTable {
    user_symbol: HashMap<String, String>,
    base_address_index: i32,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            user_symbol: HashMap::new(),
            base_address_index: 16,
        }
    }

    // if the symbol exists in the system symbol table, then return it.
    // if the symbol exists in the user_symbol, then return it.
    // if the symbol does not exist in the user_symbol, create an address based on position 16.
    pub fn get(&mut self, symbol: &str) -> String {
        let system_symbol = get_system_binary(symbol);
        if system_symbol != "_" {
            return String::from(system_symbol);
        }

        if let Some(value) = self.user_symbol.get(symbol) {
            return String::from(value);
        }

        let new_address = format!("0{:015b}", self.base_address_index);

        self.base_address_index += 1;

        self.set(String::from(symbol), new_address.clone());

        new_address
    }

    pub fn set(&mut self, symbol: String, value: String) {
        self.user_symbol.insert(symbol, value);
    }
}
