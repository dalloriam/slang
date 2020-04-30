use instructor::LabelConverter;

#[derive(Debug)]
pub enum SymbolType {
    Label,
}

#[derive(Debug)]
pub struct Symbol {
    name: String,
    offset: u32,
    symbol_type: SymbolType,
}

impl Symbol {
    pub fn new(name: String, symbol_type: SymbolType, offset: u32) -> Symbol {
        Symbol {
            name,
            symbol_type,
            offset,
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    // TODO: Maybe use hashmap here if perf. is an issue.
    symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbols: Vec::new(),
        }
    }

    pub fn add(&mut self, s: Symbol) {
        self.symbols.push(s)
    }
}

impl LabelConverter for SymbolTable {
    fn offset_of(&self, s: &str) -> Option<u32> {
        for symbol in self.symbols.iter() {
            if &symbol.name == s {
                return Some(symbol.offset);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::symbol::{Symbol, SymbolTable, SymbolType};

    #[test]
    fn test_symbol_table() {
        let mut sym = SymbolTable::new();
        let new_symbol = Symbol::new(String::from("somelabel"), SymbolTable::Label, 12);

        sym.add(new_symbol);
        assert_eq!(sym.symbols.len(), 1);

        let ofs = sym.offset_of("somelabel").unwrap();
        assert_eq!(ofs, 12);

        assert!(sym.offset_of("nonexistent").is_none());
    }
}
