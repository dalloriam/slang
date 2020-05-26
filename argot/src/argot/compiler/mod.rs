mod scope;

use assembler::Assembler;
use instructor::REGULAR_REGISTER_COUNT;

use crate::syntax::{
    arithmetic_expression::arithmetic_expression, function::function_declaration,
    ArithmeticExpression, Expression, Factor, FactorOperator, FunctionDeclaration, Statement, Term,
    TermOperator, UnaryOperator, VariableDeclaration,
};
use crate::visitor::{Visitable, Visitor};
use scope::Scope;

pub struct Compiler {
    free_registers: Vec<u8>,
    used_registers: Vec<u8>,
    stack_storecount: usize,
    total_stack_offset: usize,

    scopes: Vec<Scope>,

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
            total_stack_offset: 0,

            scopes: Vec::new(),

            assembly_buffer: Vec::new(),
        }
    }

    pub fn compile_asm(&mut self, source: &str) -> String {
        let (rest, mut f) = function_declaration(source).unwrap();
        assert_eq!(rest, "");
        self.visit_function_declaration(&mut f).unwrap();
        let program = format!(".data\n.text\n{}", self.assembly_buffer.join("\n"));

        program
    }

    pub fn compile(&mut self, source: &str) -> Vec<u8> {
        let program = self.compile_asm(source);
        Assembler::new().assemble(&program).unwrap()
    }

    fn save_val(&mut self, val: i32) {
        let latest_scope = self.scopes.last_mut().unwrap();

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
    }
    fn save_reg(&mut self, src_reg: u8) {
        let latest_scope = self.scopes.last_mut().unwrap();
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
    }

    fn pop_reg(&mut self, default: u8) -> u8 {
        let latest_scope = self.scopes.last_mut().unwrap();
        if self.stack_storecount > 0 {
            latest_scope.push_instruction(format!("pop ${}", default));
            self.stack_storecount -= 1;
            self.stack_storecount -= std::mem::size_of::<i32>();
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

        let latest_scope = self.scopes.last_mut().unwrap();

        match v {
            FactorOperator::Mult => {
                latest_scope.push_instruction(format!("mul ${} ${} $0", d2, d1))
            }
            FactorOperator::Div => latest_scope.push_instruction(format!("div ${} ${} $0", d2, d1)),
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

        let latest_scope = self.scopes.last_mut().unwrap();

        match v {
            TermOperator::Plus => latest_scope.push_instruction(format!("add ${} ${} $0", d2, d1)),
            TermOperator::Minus => latest_scope.push_instruction(format!("sub ${} ${} $0", d2, d1)),
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

    fn visit_function_declaration(&mut self, v: &mut FunctionDeclaration) -> Self::Result {
        log::debug!("Function decl");

        let new_scope = Scope::new();
        self.scopes.push(new_scope);

        for stmt in v.body.iter_mut() {
            stmt.accept(self)?;
        }

        let mut last_scope = self.scopes.pop().unwrap(); // TODO: Catch empty scopes.

        self.assembly_buffer.push(format!("{}:", v.name));

        // Generate function prelude.
        for (variable_name, stack_offset) in last_scope.local_variables().iter() {
            self.assembly_buffer
                .push(format!("sw $0 {}[$ebp]", stack_offset))
        }

        // Generate the function body (the instructions stored in the scope)
        self.assembly_buffer.extend(last_scope.take_instructions());

        // Generate the function epilogue
        for (_var_name, _stack_offset) in last_scope.local_variables().iter() {
            // Pop a word from the stack to return it to what it was before the fn.
            self.assembly_buffer.push(format!("pop $0"));
        }

        Ok(())
    }

    fn visit_statement(&mut self, stat: &mut Statement) -> Self::Result {
        match stat {
            Statement::VarDecl(decl) => decl.accept(self)?,
            _ => unimplemented!(),
        }

        Ok(())
    }

    fn visit_variable_declaration(&mut self, decl: &mut VariableDeclaration) -> Self::Result {
        {
            let latest_scope = self.scopes.last_mut().unwrap(); // TODO: Handle.
            latest_scope
                .variable_with_size(&decl.name, std::mem::size_of::<i32>())
                .unwrap();
        }

        if let Some(mut expr) = decl.expression.clone() {
            // Eval an expression. Its value is stored in the reg. at the top of the compiler
            // stack.
            {
                expr.accept(self)?;
            }

            let reg = self.pop_reg(0);

            let offset = {
                let scope = self.scopes.last_mut().unwrap();
                let var_map = scope.local_variables();
                let ofst = var_map.get(&decl.name).unwrap();
                ofst.clone()
            };

            self.scopes
                .last_mut()
                .unwrap()
                .push_instruction(format!("sw ${} {}[$ebp]", reg, offset))
        }

        Ok(())
    }

    fn visit_expression(&mut self, expr: &mut Expression) -> Self::Result {
        match expr {
            Expression::Arithmetic(arith) => arith.accept(self)?,
            _ => unimplemented!(),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
