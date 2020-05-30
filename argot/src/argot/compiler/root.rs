use std::fmt;

use assembler::{Assembler, AssemblerError};

use instructor::REGULAR_REGISTER_COUNT;

use snafu::{ResultExt, Snafu};

use crate::{
    compiler::{scope::ScopeError, Scope},
    syntax::program::program,
    visitor::Visitor,
};

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum CompileError {
    AssemblyFailed {
        source: AssemblerError,
    },
    IncompleteParse {
        source: ParseError,
    },
    MissingScope,
    NoUsedRegisters,
    UnknownIdentifier {
        name: String,
    },
    UnknownType {
        name: String,
        source: super::types::UnknownType,
    },
    VariableDeclarationError {
        source: ScopeError,
    },
}

type Result<T> = std::result::Result<T, CompileError>;

pub struct Compiler {
    free_registers: Vec<u8>,
    used_registers: Vec<u8>,
    stack_storecount: usize,

    pub scopes: Vec<Scope>,

    pub assembly_buffer: Vec<String>,
}

impl Compiler {
    pub fn new() -> Compiler {
        let mut free_reg = Vec::with_capacity(REGULAR_REGISTER_COUNT);
        // Registers 0-8 are reserved for operations 8-31 are OK for storage.
        for i in (8..REGULAR_REGISTER_COUNT).rev() {
            free_reg.push(i as u8)
        }

        Compiler {
            free_registers: free_reg,
            used_registers: Vec::new(),
            stack_storecount: 0,

            scopes: Vec::new(),

            assembly_buffer: Vec::new(),
        }
    }

    pub fn compile_asm(&mut self, source: &str) -> Result<String> {
        let (rest, mut p) = program(source)
            .map_err(|e| ParseError {
                message: e.to_string(),
            })
            .context(IncompleteParse)?;

        assert_eq!(rest, "");
        println!("{:?}", p);
        self.visit_program(&mut p)?;
        let program = format!(
            ".data\n.text\njmp @main\n{}",
            self.assembly_buffer.join("\n")
        );

        Ok(program)
    }

    pub fn compile(&mut self, source: &str) -> Result<Vec<u8>> {
        let program = self.compile_asm(source)?;
        Assembler::new().assemble(&program).context(AssemblyFailed)
    }

    pub fn save_val(&mut self, val: i32) -> Result<()> {
        let latest_scope = self.scopes.last_mut().ok_or(CompileError::MissingScope)?;

        match self.free_registers.pop() {
            Some(reg) => {
                // A storage register is available -- save it there.
                latest_scope.push_instruction(format!("ld ${} {:#06x}", reg, val));
                self.used_registers.push(reg);
            }
            None => {
                // No storage register is available. Store the value in the vm stack.
                latest_scope.push_instruction(format!("ld $7 {:#06x}\npush $7", val));
                self.stack_storecount += 1;
                self.stack_storecount += std::mem::size_of::<i32>();
            }
        }
        Ok(())
    }

    pub fn save_reg(&mut self, src_reg: u8) -> Result<()> {
        let latest_scope = self.scopes.last_mut().ok_or(CompileError::MissingScope)?;
        match self.free_registers.pop() {
            Some(reg) => {
                // Found a storage register, copy there.
                latest_scope.push_instruction(format!("move ${} ${}", src_reg, reg));
                self.used_registers.push(reg);
            }
            None => {
                // No storage register -- Store on stack instead.
                latest_scope.push_instruction(format!("push ${}", src_reg));
                self.stack_storecount += 1;
                self.stack_storecount += std::mem::size_of::<i32>();
            }
        }
        Ok(())
    }

    pub fn pop_reg(&mut self, default: u8) -> Result<u8> {
        let latest_scope = self.scopes.last_mut().ok_or(CompileError::MissingScope)?;
        let reg = if self.stack_storecount > 0 {
            latest_scope.push_instruction(format!("pop ${}", default));
            self.stack_storecount -= 1;
            self.stack_storecount -= std::mem::size_of::<i32>();
            default
        } else {
            debug_assert!(!self.used_registers.is_empty());
            let r = self
                .used_registers
                .pop()
                .ok_or(CompileError::NoUsedRegisters)?;
            self.free_registers.push(r);
            r
        };
        Ok(reg)
    }

    pub fn current_scope(&self) -> Result<&Scope> {
        self.scopes.last().ok_or(CompileError::MissingScope)
    }

    pub fn current_scope_mut(&mut self) -> Result<&mut Scope> {
        self.scopes.last_mut().ok_or(CompileError::MissingScope)
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new())
    }

    pub fn pop_scope(&mut self) -> Result<Scope> {
        self.scopes.pop().ok_or(CompileError::MissingScope)
    }
}

impl Default for Compiler {
    fn default() -> Compiler {
        Compiler::new()
    }
}
