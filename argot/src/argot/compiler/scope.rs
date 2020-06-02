use std::cmp;
use std::collections::HashMap;
use std::mem;

use snafu::{ensure, Snafu};

#[derive(Debug, Snafu)]
pub enum ScopeError {
    VariableAlreadyDefined,
}

type Result<T> = std::result::Result<T, ScopeError>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub offset: usize,
    pub var_type: String,
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

    pub fn push_instruction(&mut self, instr: String) {
        self.instruction_buffer.push(instr);
    }

    pub fn local_variables(&self) -> &HashMap<String, Variable> {
        &self.local_variables
    }

    pub fn sorted_variables(&self) -> Vec<&Variable> {
        let mut refs = Vec::new();

        for var_name in self.variables_insert_order.iter() {
            refs.push(self.local_variables.get(var_name).unwrap())
        }

        refs
    }

    pub fn variable_with_size(
        &mut self,
        variable_name: &str,
        var_type: String,
        size: usize,
    ) -> Result<()> {
        ensure!(
            !self.local_variables.contains_key(variable_name),
            VariableAlreadyDefined
        );

        self.variables_insert_order
            .push(String::from(variable_name));
        self.local_variables.insert(
            String::from(variable_name),
            Variable {
                name: String::from(variable_name),
                offset: self.local_stack_offset,
                var_type,
            },
        );
        self.local_stack_offset += size;

        Ok(())
    }
}
