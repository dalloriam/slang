use std::collections::HashMap;
use std::mem;

use snafu::ensure;

use crate::compiler::error::*;
use crate::syntax::types::{Argument, FunctionDeclaration, Program};

#[derive(Clone, Debug)]
pub struct FunctionDecl {
    pub name: String,
    pub arguments: Vec<Argument>,
    pub return_type: Option<String>,
}

impl From<FunctionDeclaration> for FunctionDecl {
    fn from(d: FunctionDeclaration) -> Self {
        FunctionDecl {
            name: d.name,
            return_type: d.return_type,
            arguments: d.args.arguments,
        }
    }
}

pub struct FirstPassOutput {
    pub functions: HashMap<String, FunctionDecl>,
}

pub struct FirstPassVisitor {
    functions: HashMap<String, FunctionDecl>,
}

impl FirstPassVisitor {
    pub fn new() -> FirstPassVisitor {
        FirstPassVisitor {
            functions: HashMap::new(),
        }
    }

    fn visit_function_declaration(&mut self, decl: FunctionDeclaration) {
        self.functions
            .insert(decl.name.clone(), FunctionDecl::from(decl));
    }

    pub fn apply(&mut self, program: &mut Program) -> Result<FirstPassOutput> {
        for (_function_name, function_decl) in program.functions.iter_mut() {
            self.visit_function_declaration(function_decl.clone());
        }
        ensure!(self.functions.contains_key("main"), MissingEntryPoint);

        let mut functions = HashMap::new();

        mem::swap(&mut functions, &mut self.functions);

        Ok(FirstPassOutput { functions })
    }
}

impl Default for FirstPassVisitor {
    fn default() -> Self {
        Self::new()
    }
}
