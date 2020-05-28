use crate::{
    compiler::Compiler,
    syntax::{
        ArithmeticExpression, Expression, Factor, FactorOperator, FunctionDeclaration, Program,
        Statement, Term, TermOperator, UnaryOperator, VariableDeclaration,
    },
    visitor::{Visitable, Visitor},
};

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
            Factor::Identifier(var_name) => {
                let scope = self.scopes.last_mut().unwrap();
                // Load the val. of the variable from the runtime stack & push the register address
                // to the compiler stack.
                let var_offset = *scope.local_variables().get(var_name).unwrap(); // TODO: Handle errors.
                scope.push_instruction(format!("lw $0 {}[$ebp]", var_offset));
                self.save_reg(0);
                Ok(())
            }
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
        self.push_scope();

        for stmt in v.body.iter_mut() {
            stmt.accept(self)?;
        }

        let mut last_scope = self.pop_scope();

        self.assembly_buffer.push(format!("{}:", v.name));

        // Generate function prelude.
        for (_variable_name, stack_offset) in last_scope.local_variables().iter() {
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

        if v.name == "main" {
            // Generate exit syscall after main.
            self.assembly_buffer.push(String::from("ld $v0 0x0002"));
            self.assembly_buffer.push(String::from("syscall"))
        } else {
            self.assembly_buffer.push(String::from("ret"))
        }

        Ok(())
    }

    fn visit_statement(&mut self, stat: &mut Statement) -> Self::Result {
        match stat {
            Statement::VarDecl(decl) => decl.accept(self)?,
            Statement::Return(expr_maybe) => {
                if let Some(expr) = expr_maybe {
                    expr.accept(self)?;
                }
                self.current_scope_mut()
                    .push_instruction(String::from("ret"))
            }
            Statement::Expr(expr) => {
                // Evaluate the expr & do nothing else.
                expr.accept(self)?;
            }
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
            Expression::Identifier(var_name) => {
                let scope = self.scopes.last_mut().unwrap();
                // Load the val. of the variable from the runtime stack & push the register address
                // to the compiler stack.
                let var_offset = *scope.local_variables().get(var_name).unwrap(); // TODO: Handle errors.
                scope.push_instruction(format!("lw $0 {}[$ebp]", var_offset));
                self.save_reg(0);
            }
            Expression::FunctionCall(fn_name) => {
                let scope = self.current_scope_mut();
                scope.push_instruction(format!("call @{}", fn_name)); // TODO: Validate that fn exists.
            }
        }
        Ok(())
    }

    fn visit_program(&mut self, v: &mut Program) -> Self::Result {
        for (_fn_name, fn_decl) in v.functions.iter_mut() {
            // TODO: Index function names to spot duplicates & validate function calls.
            fn_decl.accept(self)?;
        }

        Ok(())
    }
}
