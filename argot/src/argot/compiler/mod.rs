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

    assembly_buffer: Vec<String>,
}

impl Compiler {
    pub fn new() -> Compiler {
        let mut free_reg = Vec::with_capacity(REGULAR_REGISTER_COUNT);
        for i in 0..REGULAR_REGISTER_COUNT {
            free_reg.push(i as u8)
        }

        Compiler {
            free_registers: free_reg,
            used_registers: Vec::new(),

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
                let register = self.free_registers.pop().unwrap();
                self.used_registers.push(register);
                self.assembly_buffer
                    .push(format!("ld ${} {:#06x}", register, i));
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
        let result_register = self.free_registers.pop().unwrap();
        let data_register_2 = self.used_registers.pop().unwrap();
        let data_register_1 = self.used_registers.pop().unwrap();
        match v {
            FactorOperator::Mult => self.assembly_buffer.push(format!(
                "mul ${} ${} ${}",
                data_register_1, data_register_2, result_register
            )),
            FactorOperator::Div => self.assembly_buffer.push(format!(
                "div ${} ${} ${}",
                data_register_1, data_register_2, result_register
            )),
            FactorOperator::Unknown => panic!("unknown factor operator"),
        }
        self.free_registers.push(data_register_2);
        self.free_registers.push(data_register_1);
        self.used_registers.push(result_register);

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
        let result_register = self.free_registers.pop().unwrap();
        let data_register_2 = self.used_registers.pop().unwrap();
        let data_register_1 = self.used_registers.pop().unwrap();
        match v {
            TermOperator::Plus => self.assembly_buffer.push(format!(
                "add ${} ${} ${}",
                data_register_1, data_register_2, result_register
            )),
            TermOperator::Minus => self.assembly_buffer.push(format!(
                "sub ${} ${} ${}",
                data_register_1, data_register_2, result_register
            )),
            TermOperator::Unknown => panic!("unknown term operator"),
        }
        self.free_registers.push(data_register_2);
        self.free_registers.push(data_register_1);
        self.used_registers.push(result_register);
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
