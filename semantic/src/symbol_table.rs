use std::collections::HashMap;

pub trait Symbol {
    fn identifier(&self) -> &str;
    fn symbol_type(&self) -> &str;
}

pub struct Function {
    pub identifier: String,
    pub return_type: String,
    pub parameters: Vec<Variable>,
}

impl Symbol for Function {
    fn identifier(&self) -> &str {
        &self.identifier
    }
    
    fn symbol_type(&self) -> &str {
        &self.return_type
    }
}

pub struct Variable {
    pub identifier: String,
    pub var_type: String,
}

impl Symbol for Variable {
    fn identifier(&self) -> &str {
        &self.identifier
    }
    
    fn symbol_type(&self) -> &str {
        &self.var_type
    }
}

#[derive(Debug)]
pub enum SymbolTableError {
    SymbolNotFound(String),
    SymbolAlreadyExists(String),
}

pub struct SymbolTable {
    upper_scope: Option<Box<SymbolTable>>,
    symbols: HashMap<String, Box<dyn Symbol>>,
}

impl SymbolTable {
    pub fn new(upper_scope: Option<SymbolTable>) -> Self {
        Self {
            upper_scope: upper_scope.map(Box::new),
            symbols: HashMap::new(),
        }
    }

    pub fn exists(&self, identifier: &str) -> bool {
        if self.symbols.contains_key(identifier) {
            true
        } else if let Some(ref parent) = self.upper_scope {
            parent.exists(identifier)
        } else {
            false
        }
    }

    pub fn add(&mut self, symbol: Box<dyn Symbol>) -> Result<(), SymbolTableError> {
        let identifier = symbol.identifier().to_string();
        if self.symbols.contains_key(&identifier) {
            Err(SymbolTableError::SymbolAlreadyExists(identifier))
        } else {
            self.symbols.insert(identifier, symbol);
            Ok(())
        }
    }

    pub fn get(&self, identifier: &str) -> Result<&dyn Symbol, SymbolTableError> {
        if let Some(symbol) = self.symbols.get(identifier) {
            Ok(symbol.as_ref())
        } else if let Some(ref parent) = self.upper_scope {
            parent.get(identifier)
        } else {
            Err(SymbolTableError::SymbolNotFound(identifier.to_string()))
        }
    }    
}
