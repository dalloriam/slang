use assembler::Assembler;
use instructor::REGULAR_REGISTER_COUNT;

use crate::syntax::{
    arithmetic_expression::arithmetic_expression, ArithmeticExpression, Factor, FactorOperator,
    Term, TermOperator, UnaryOperator,
};
use crate::visitor::{Visitable, Visitor};

pub struct Compiler {
    free_registers: Vec<u8>,
    used_registers: Vec<u8>,
    stack_storecount: usize,

    assembly_buffer: Vec<String>,
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

            assembly_buffer: Vec::new(),
        }
    }

    pub fn compile_asm(&mut self, source: &str) -> String {
        let (rest, mut expr) = arithmetic_expression(source).unwrap();
        assert_eq!(rest, "");
        self.visit_arithmetic_expression(&mut expr).unwrap();
        let program = format!(".data\n.text\n{}", self.assembly_buffer.join("\n"));

        program
    }

    pub fn compile(&mut self, source: &str) -> Vec<u8> {
        let program = self.compile_asm(source);
        Assembler::new().assemble(&program).unwrap()
    }

    fn save_val(&mut self, val: i32) {
        match self.free_registers.pop() {
            Some(reg) => {
                // A storage register is available -- save it there.
                self.assembly_buffer
                    .push(format!("ld ${} {:#06x}", reg, val));
                self.used_registers.push(reg);
            }
            None => {
                // No storage register is available. Store the value in the vm stack.
                self.assembly_buffer
                    .push(format!("ld $7 {:#06x}\npush $7", val));
                self.stack_storecount += 1;
            }
        }
    }
    fn save_reg(&mut self, src_reg: u8) {
        match self.free_registers.pop() {
            Some(reg) => {
                // Found a storage register, copy there.
                self.assembly_buffer
                    .push(format!("move ${} ${}", src_reg, reg));
                self.used_registers.push(reg);
            }
            None => {
                // No storage register -- Store on stack instead.
                self.assembly_buffer.push(format!("push ${}", src_reg));
                self.stack_storecount += 1;
            }
        }
    }

    fn pop_reg(&mut self, default: u8) -> u8 {
        if self.stack_storecount > 0 {
            self.assembly_buffer.push(format!("pop ${}", default));
            self.stack_storecount -= 1;
            default
        } else {
            debug_assert!(!self.used_registers.is_empty());
            let r = self.used_registers.pop().unwrap();
            self.free_registers.push(r);
            r
        }
    }
}

impl Default for Compiler {
    fn default() -> Compiler {
        Compiler::new()
    }
}

impl Visitor for Compiler {
    type Result = std::result::Result<(), std::convert::Infallible>;

    fn visit_arithmetic_expression(&mut self, v: &mut ArithmeticExpression) -> Self::Result {
        log::debug!("arithmetic expression");

        v.root_term.accept(self)?;

        for (term_op, term) in v.trail.iter_mut() {
            term.accept(self)?;
            term_op.accept(self)?;
        }
        Ok(())
    }
    fn visit_factor(&mut self, v: &mut Factor) -> Self::Result {
        log::debug!("factor");
        match v {
            Factor::Integer(i) => {
                self.save_val(*i);
                Ok(())
            }
            Factor::Unary(op, f) => {
                op.accept(self)?;
                f.accept(self)
            }
            Factor::Expression(e) => e.accept(self),
        }
    }
    fn visit_factor_operator(&mut self, v: &mut FactorOperator) -> Self::Result {
        log::debug!("factor operator");

        let d1 = self.pop_reg(0);
        let d2 = self.pop_reg(1);

        match v {
            FactorOperator::Mult => self.assembly_buffer.push(format!("mul ${} ${} $0", d2, d1)),
            FactorOperator::Div => self.assembly_buffer.push(format!("div ${} ${} $0", d2, d1)),
            FactorOperator::Unknown => panic!("unknown factor operator"),
        }

        self.save_reg(0);

        Ok(())
    }
    fn visit_term(&mut self, v: &mut Term) -> Self::Result {
        log::debug!("term");

        v.root_factor.accept(self)?;

        for (op, t) in v.trail.iter_mut() {
            t.accept(self)?;

            op.accept(self)?;
        }
        Ok(())
    }
    fn visit_term_operator(&mut self, v: &mut TermOperator) -> Self::Result {
        log::debug!("term operator");
        let d1 = self.pop_reg(0);
        let d2 = self.pop_reg(1);

        match v {
            TermOperator::Plus => self.assembly_buffer.push(format!("add ${} ${} $0", d2, d1)),
            TermOperator::Minus => self.assembly_buffer.push(format!("sub ${} ${} $0", d2, d1)),
            TermOperator::Unknown => panic!("unknown term operator"),
        }

        self.save_reg(0);
        Ok(())
    }
    fn visit_unary_operator(&mut self, v: &mut UnaryOperator) -> Self::Result {
        log::debug!("unary operator");
        match v {
            UnaryOperator::Plus => {}
            UnaryOperator::Minus => {}
            UnaryOperator::Unknown => panic!("unknown unary operator"),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
