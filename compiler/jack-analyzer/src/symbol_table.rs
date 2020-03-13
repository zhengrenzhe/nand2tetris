use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct SymbolTable {
    field_index: i32,
    static_index: i32,
    argument_index: i32,
    local_index: i32,

    store: HashMap<String, (SymbolType, SymbolKind, i32)>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            field_index: -1,
            static_index: -1,
            argument_index: -1,
            local_index: -1,
            store: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: String, symbol_type: SymbolType, symbol_kind: SymbolKind) {
        let index = match symbol_kind {
            SymbolKind::Field => {
                self.field_index += 1;
                self.field_index
            }
            SymbolKind::Static => {
                self.static_index += 1;
                self.static_index
            }
            SymbolKind::Argument => {
                self.argument_index += 1;
                self.argument_index
            }
            SymbolKind::Local => {
                self.local_index += 1;
                self.local_index
            }
        };
        self.store.insert(name, (symbol_type, symbol_kind, index));
    }

    pub fn get(&self, name: &str) -> Option<&(SymbolType, SymbolKind, i32)> {
        self.store.get(name)
    }
}

#[derive(Debug, PartialEq)]
pub enum SymbolType {
    Int,
    Char,
    Boolean,
    Class(String),
}

#[derive(Debug, PartialEq)]
pub enum SymbolKind {
    Field,
    Static,
    Argument,
    Local,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_table() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add(String::from("a"), SymbolType::Int, SymbolKind::Static);
        symbol_table.add(String::from("b"), SymbolType::Int, SymbolKind::Static);
        symbol_table.add(String::from("c"), SymbolType::Int, SymbolKind::Field);
        symbol_table.add(String::from("d"), SymbolType::Int, SymbolKind::Field);
        symbol_table.add(String::from("e"), SymbolType::Int, SymbolKind::Argument);
        symbol_table.add(String::from("f"), SymbolType::Int, SymbolKind::Argument);
        symbol_table.add(String::from("g"), SymbolType::Int, SymbolKind::Local);
        symbol_table.add(String::from("h"), SymbolType::Int, SymbolKind::Local);

        assert_eq!(
            symbol_table.get("a"),
            Some(&(SymbolType::Int, SymbolKind::Static, 0))
        );
        assert_eq!(
            symbol_table.get("b"),
            Some(&(SymbolType::Int, SymbolKind::Static, 1))
        );
        assert_eq!(
            symbol_table.get("c"),
            Some(&(SymbolType::Int, SymbolKind::Field, 0))
        );
        assert_eq!(
            symbol_table.get("d"),
            Some(&(SymbolType::Int, SymbolKind::Field, 1))
        );
        assert_eq!(
            symbol_table.get("e"),
            Some(&(SymbolType::Int, SymbolKind::Argument, 0))
        );
        assert_eq!(
            symbol_table.get("f"),
            Some(&(SymbolType::Int, SymbolKind::Argument, 1))
        );
        assert_eq!(
            symbol_table.get("g"),
            Some(&(SymbolType::Int, SymbolKind::Local, 0))
        );
        assert_eq!(
            symbol_table.get("h"),
            Some(&(SymbolType::Int, SymbolKind::Local, 1))
        );
    }
}
