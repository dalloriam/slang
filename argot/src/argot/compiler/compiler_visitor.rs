use std::convert::TryFrom;

use snafu::ResultExt;

use crate::{
    compiler::{root::*, scope::Variable, types},
    syntax::{
        ArithmeticExpression, Atom, Expression, Factor, FactorOperator, FunctionDeclaration,
        Program, Statement, Term, TermOperator, UnaryOperator, VariableDeclaration,
    },
    visitor::{Visitable, Visitor},
};

impl Visitor for Compiler {
    type Result = std::result::Result<(), CompileError>;

    fn visit_arithmetic_expression(&mut self, v: &mut ArithmeticExpression) -> Self::Result {
        log::debug!("arithmetic expression");

        v.root_term.accept(self)?;

        for (term_op, term) in v.trail.iter_mut() {
            term.accept(self)?;
            term_op.accept(self)?;
        }
        Ok(())
    }
    fn visit_atom(&mut self, v: &mut Atom) -> Self::Result {
        match v {
            Atom::Integer(i) => {
                log::debug!("int");
                self.save_val(*i)?;
                Ok(())
            }
            Atom::Identifier(var_name) => {
                let scope = self.scopes.last_mut().ok_or(CompileError::MissingScope)?;

                // Load the val. of the variable from the runtime stack & push the register address
                // to the compiler stack.
                let var_offset = scope
                    .local_variables()
                    .get(var_name)
                    .ok_or(CompileError::UnknownIdentifier {
                        name: var_name.clone(),
                    })?
                    .offset;

                // TODO: Load the same size that was written, for obvious reasons.
                scope.push_instruction(format!("lw $0 {}[$ebp]", var_offset));
                self.save_reg(0)?;
                Ok(())
            }
            Atom::Boolean(bool_val) => {
                if *bool_val {
                    self.save_val(1)
                } else {
                    self.save_val(0)
                }
            }
        }
    }

    fn visit_factor(&mut self, v: &mut Factor) -> Self::Result {
        log::debug!("factor");
        match v {
            Factor::Unary(op, f) => {
                log::debug!("unary");
                f.accept(self)?;
                op.accept(self)
            }
            Factor::Expression(e) => e.accept(self),
            Factor::Atom(at) => at.accept(self),
        }
    }
    fn visit_factor_operator(&mut self, v: &mut FactorOperator) -> Self::Result {
        log::debug!("factor operator");

        let d1 = self.pop_reg(0)?;
        let d2 = self.pop_reg(1)?;

        let latest_scope = self.scopes.last_mut().ok_or(CompileError::MissingScope)?;

        match v {
            FactorOperator::Mult => {
                latest_scope.push_instruction(format!("mul ${} ${} $0", d2, d1))
            }
            FactorOperator::Div => latest_scope.push_instruction(format!("div ${} ${} $0", d2, d1)),
            FactorOperator::Unknown => panic!("unknown factor operator"),
        }

        self.save_reg(0)?;

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
        let d1 = self.pop_reg(0)?;
        let d2 = self.pop_reg(1)?;

        let latest_scope = self.scopes.last_mut().ok_or(CompileError::MissingScope)?;

        match v {
            TermOperator::Plus => latest_scope.push_instruction(format!("add ${} ${} $0", d2, d1)),
            TermOperator::Minus => latest_scope.push_instruction(format!("sub ${} ${} $0", d2, d1)),
            TermOperator::Unknown => panic!("unknown term operator"),
        }

        self.save_reg(0)?;
        Ok(())
    }
    fn visit_unary_operator(&mut self, v: &mut UnaryOperator) -> Self::Result {
        log::debug!("unary operator");
        match v {
            UnaryOperator::Plus => {
                // Do nothing.
            }
            UnaryOperator::Minus => {
                // Issue a neg instruction in current scope.
                let reg = self.pop_reg(0)?;
                self.current_scope_mut()?
                    .push_instruction(format!("neg ${}", reg));
                self.save_reg(reg)?;
            }
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

        let mut last_scope = self.pop_scope()?;

        self.assembly_buffer.push(format!("{}:", v.name));

        // Generate function prelude.
        // This trick guarantees stack variable declaration order.
        let mut offsets: Vec<Variable> = last_scope
            .local_variables()
            .iter()
            .map(|(_n, v)| v.clone())
            .collect();
        offsets.sort();

        for var in offsets.iter() {
            // Allocate the stack space for this variable.
            // TODO: Properly allocate stack space according to type size w/ integer division.
            let variable_type =
                types::BuiltInType::try_from(var.var_type.clone()).context(UnknownType {
                    name: var.var_type.clone(),
                })?;
            let sz = variable_type.alloc_size();
            if sz == 4 {
                // TODO: Add WORD_LENGTH constant.
                self.assembly_buffer
                    .push(format!("sw $0 {}[$ebp]", var.offset))
            } else if sz == 1 {
                self.assembly_buffer
                    .push(format!("sb $0 {}[$ebp]", var.offset))
            } else {
                panic!("Bad alloc size")
            }
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
                self.current_scope_mut()?
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
        let variable_type =
            types::BuiltInType::try_from(decl.var_type.clone()).context(UnknownType {
                name: decl.var_type.clone(),
            })?;

        {
            let latest_scope = self.scopes.last_mut().ok_or(CompileError::MissingScope)?;

            latest_scope
                .variable_with_size(
                    &decl.name,
                    decl.var_type.clone(),
                    variable_type.alloc_size(),
                )
                .context(VariableDeclarationError)?;
        }

        if let Some(mut expr) = decl.expression.clone() {
            // Eval an expression. Its value is stored in the reg. at the top of the compiler
            // stack.
            {
                expr.accept(self)?;
            }

            let reg = self.pop_reg(0)?;

            let offset = {
                let scope = self.scopes.last_mut().ok_or(CompileError::MissingScope)?;
                let var_map = scope.local_variables();
                var_map
                    .get(&decl.name)
                    .ok_or(CompileError::UnknownIdentifier {
                        name: decl.name.clone(),
                    })?
                    .offset
            };

            // Allocate the stack space for this variable.
            // TODO: Properly allocate stack space according to type size w/ integer division.
            let sz = variable_type.alloc_size();
            if sz == 4 {
                // TODO: Add WORD_LENGTH constant.
                self.scopes
                    .last_mut()
                    .ok_or(CompileError::MissingScope)?
                    .push_instruction(format!("sw ${} {}[$ebp]", reg, offset))
            } else if sz == 1 {
                self.scopes
                    .last_mut()
                    .ok_or(CompileError::MissingScope)?
                    .push_instruction(format!("sb ${} {}[$ebp]", reg, offset))
            } else {
                panic!("Bad alloc size")
            }
        }

        Ok(())
    }

    fn visit_expression(&mut self, expr: &mut Expression) -> Self::Result {
        match expr {
            Expression::Arithmetic(arith) => arith.accept(self)?,
            Expression::Identifier(var_name) => {
                let scope = self.scopes.last_mut().ok_or(CompileError::MissingScope)?;
                // Load the val. of the variable from the runtime stack & push the register address
                // to the compiler stack.
                let offset = scope
                    .local_variables()
                    .get(var_name)
                    .ok_or(CompileError::UnknownIdentifier {
                        name: var_name.clone(),
                    })?
                    .offset;
                scope.push_instruction(format!("lw $0 {}[$ebp]", offset));
                self.save_reg(0)?;
            }
            Expression::FunctionCall(fn_name) => {
                let scope = self.current_scope_mut()?;
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
