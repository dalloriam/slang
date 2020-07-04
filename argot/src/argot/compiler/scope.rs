use std::cmp;
use std::collections::HashMap;
use std::mem;

use snafu::ensure;

use crate::compiler::error::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub offset: i32,
    pub var_type: String,
    pub size: usize,
}

impl cmp::PartialOrd for Variable {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.offset.cmp(&other.offset))
    }
}

impl cmp::Ord for Variable {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.offset.cmp(&other.offset)
    }
}

pub struct Scope {
    local_stack_offset: usize,
    local_variables: HashMap<String, Variable>,
    variables_insert_order: Vec<String>,
    instruction_buffer: Vec<String>,
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::new()
    }
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            local_stack_offset: 0,
            local_variables: HashMap::new(),
            variables_insert_order: Vec::new(),
            instruction_buffer: Vec::new(),
        }
    }

    pub fn take_instructions(&mut self) -> Vec<String> {
        let mut new_v = Vec::new();
        mem::swap(&mut self.instruction_buffer, &mut new_v);
        new_v
    }

    pub fn extend(&mut self, other: &mut Scope) {
        self.instruction_buffer.extend(other.take_instructions());
    }

    pub fn push_instruction(&mut self, instr: String) {
        self.instruction_buffer.push(instr);
    }

    pub fn get_variable(&self, name: &str) -> Result<&Variable> {
        self.local_variables
            .get(name)
            .ok_or(CompileError::UnknownIdentifier {
                name: String::from(name),
            })
    }

    pub fn sorted_variables(&self) -> Vec<&Variable> {
        let mut refs = Vec::new();

        for var_name in self.variables_insert_order.iter() {
            refs.push(self.local_variables.get(var_name).unwrap())
        }

        refs
    }

    pub fn capture(
        &mut self,
        variable_name: String,
        var_type: String,
        size: usize,
        offset: i32,
    ) -> Result<()> {
        ensure!(
            !self.local_variables.contains_key(&variable_name),
            VariableAlreadyDefined {
                name: String::from(variable_name)
            }
        );

        let v = Variable {
            name: variable_name.clone(),
            offset,
            size,
            var_type,
        };
        self.local_variables.insert(variable_name, v);
        Ok(())
    }

    pub fn variable_with_size(
        &mut self,
        variable_name: &str,
        var_type: String,
        size: usize,
    ) -> Result<Variable> {
        ensure!(
            !self.local_variables.contains_key(variable_name),
            VariableAlreadyDefined {
                name: String::from(variable_name)
            }
        );

        self.variables_insert_order
            .push(String::from(variable_name));

        let v = Variable {
            name: String::from(variable_name),
            offset: self.local_stack_offset as i32,
            size,
            var_type,
        };

        self.local_variables
            .insert(String::from(variable_name), v.clone());
        self.local_stack_offset += size;

        Ok(v)
    }
}

pub struct ScopeManager {
    scopes: Vec<Scope>,
}

impl ScopeManager {
    pub fn new() -> ScopeManager {
        let root_scope = Scope::new();
        ScopeManager {
            scopes: vec![root_scope],
        }
    }

    pub fn len(&self) -> usize {
        self.scopes.len()
    }

    pub fn push(&mut self) {
        let new_scope = Scope::new();
        self.scopes.push(new_scope);
    }

    pub fn pop(&mut self) -> Result<Scope> {
        self.scopes.pop().ok_or(CompileError::MissingScope)
    }

    pub fn current(&self) -> Result<&Scope> {
        self.scopes.last().ok_or(CompileError::MissingScope)
    }

    pub fn current_mut(&mut self) -> Result<&mut Scope> {
        self.scopes.last_mut().ok_or(CompileError::MissingScope)
    }
}
