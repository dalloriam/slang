use std::collections::HashMap;
use std::mem;

use snafu::{ensure, Snafu};

#[derive(Debug, Snafu)]
pub enum ScopeError {
    VariableAlreadyDefined,
}

type Result<T> = std::result::Result<T, ScopeError>;

pub struct Scope {
    local_stack_offset: usize,
    local_variables: HashMap<String, usize>,
    instruction_buffer: Vec<String>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            local_stack_offset: 0,
            local_variables: HashMap::new(),
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

    pub fn local_variables(&self) -> &HashMap<String, usize> {
        &self.local_variables
    }

    pub fn variable_with_size(&mut self, variable_name: &str, size: usize) -> Result<()> {
        ensure!(
            !self.local_variables.contains_key(variable_name),
            VariableAlreadyDefined
        );

        self.local_variables
            .insert(String::from(variable_name), self.local_stack_offset);
        self.local_stack_offset += size;

        Ok(())
    }
}
