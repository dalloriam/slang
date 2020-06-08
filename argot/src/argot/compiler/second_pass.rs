use std::collections::HashMap;
use std::convert::TryFrom;

use instructor::REGULAR_REGISTER_COUNT;

use snafu::{ensure, ResultExt};

use crate::{
    compiler::{emit, error::*, first_pass::FunctionDecl, scope::ScopeManager, typing},
    syntax::types::*,
    visitor::{Visitable, Visitor},
};

pub struct SecondPassVisitor {
    free_registers: Vec<u8>,
    functions: HashMap<String, FunctionDecl>,
    scopes: ScopeManager,
    stack_size_tracker: usize,
    type_stack: Vec<String>,
    used_registers: Vec<u8>,
}

impl SecondPassVisitor {
    pub fn new(functions: HashMap<String, FunctionDecl>) -> SecondPassVisitor {
        let mut free_registers = Vec::with_capacity(REGULAR_REGISTER_COUNT);

        for i in (8..REGULAR_REGISTER_COUNT).rev() {
            free_registers.push(i as u8);
        }

        SecondPassVisitor {
            free_registers,
            functions,
            scopes: ScopeManager::new(),
            stack_size_tracker: 0,
            type_stack: Vec::new(),
            used_registers: Vec::new(),
        }
    }

    pub fn apply(&mut self, program: &mut Program) -> Result<String> {
        program.accept(self)?;
        debug_assert_eq!(self.scopes.len(), 1);
        let instr = self.scopes.current_mut()?.take_instructions();
        let program = format!(".data\n.text\njmp @main\n{}", instr.join("\n"));
        Ok(program)
    }

    fn push_type(&mut self, t: String) {
        self.type_stack.push(t);
    }

    fn pop_type(&mut self) -> Result<String> {
        self.type_stack.pop().ok_or(CompileError::MissingType)
    }

    fn save_val(&mut self, val: i32) -> Result<()> {
        match self.free_registers.pop() {
            Some(register) => {
                emit::save_to_register(val, register, &mut self.scopes)?;
                self.used_registers.push(register);
            }
            None => {
                emit::save_to_register(val, 7, &mut self.scopes)?;
                emit::stack_push_word(7, &mut self.scopes)?;
                self.stack_size_tracker += 1;
            }
        }

        Ok(())
    }

    pub fn get_writeable_register(&mut self) -> Result<u8> {
        match self.free_registers.pop() {
            Some(i) => {
                self.used_registers.push(i);
                Ok(i)
            }
            None => Ok(0), // TODO: Use 1-8
        }
    }

    pub fn save_reg_maybe(&mut self, register: u8) -> Result<()> {
        if register < 8 {
            self.save_reg(register)?;
        }
        Ok(())
    }

    fn save_reg(&mut self, src_reg: u8) -> Result<()> {
        match self.free_registers.pop() {
            Some(reg) => {
                emit::mov(src_reg, reg, &mut self.scopes)?;
                self.used_registers.push(reg);
            }
            None => {
                emit::stack_push_word(src_reg, &mut self.scopes)?;
                self.stack_size_tracker += 1;
            }
        }
        Ok(())
    }

    fn pop_reg(&mut self, default: u8) -> Result<u8> {
        let reg = if self.stack_size_tracker > 0 {
            emit::stack_pop_word(default, &mut self.scopes)?;
            self.stack_size_tracker -= 1;
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
}

impl Visitor for SecondPassVisitor {
    type Result = Result<()>;

    fn visit_expression(&mut self, v: &mut Expression) -> Self::Result {
        v.root_term.accept(self)?;

        for (term_operator, term) in v.trail.iter_mut() {
            term.accept(self)?;
            term_operator.accept(self)?;
        }

        Ok(())
    }

    fn visit_factor(&mut self, v: &mut Factor) -> Self::Result {
        match v {
            Factor::FunctionCall(fn_call) => fn_call.accept(self),
            Factor::Atomic(atom) => atom.accept(self),
            Factor::Expression(expr) => expr.accept(self),
            Factor::Unary(unary_op, factor) => {
                factor.accept(self)?;
                unary_op.accept(self)
            }
            Factor::IfExpression(if_expr) => if_expr.accept(self),
        }
    }

    fn visit_factor_operator(&mut self, v: &mut FactorOperator) -> Self::Result {
        // Typecheck.
        let t1 = self.pop_type()?;
        let t2 = self.pop_type()?;
        typing::typecheck_binary_operator::<FactorOperator>(&t1, &t2)?;

        // Execution.
        let o1 = self.pop_reg(0)?;
        let o2 = self.pop_reg(1)?;
        let operation = match v {
            FactorOperator::Mult => "mul",
            FactorOperator::Div => "div",
            FactorOperator::Unknown => panic!("Unknown operator"),
        };

        let result_register = self.get_writeable_register()?;
        emit::binary_operation(operation, o2, o1, result_register, &mut self.scopes)?;
        self.save_reg_maybe(result_register)?;

        // Push type of resulting value.
        self.push_type(t1);
        Ok(())
    }

    fn visit_term(&mut self, v: &mut Term) -> Self::Result {
        v.root_factor.accept(self)?;

        for (operator, factor) in v.trail.iter_mut() {
            factor.accept(self)?;
            operator.accept(self)?;
        }

        Ok(())
    }

    fn visit_term_operator(&mut self, v: &mut TermOperator) -> Self::Result {
        // Typecheck.
        let t1 = self.pop_type()?;
        let t2 = self.pop_type()?;
        typing::typecheck_binary_operator::<FactorOperator>(&t1, &t2)?;

        // Execution
        let o1 = self.pop_reg(0)?;
        let o2 = self.pop_reg(1)?;
        let operation = match v {
            TermOperator::Plus => "add",
            TermOperator::Minus => "sub",
            TermOperator::Unknown => panic!("Unknown operator"),
        };

        let result_register = self.get_writeable_register()?;
        emit::binary_operation(operation, o2, o1, result_register, &mut self.scopes)?;
        self.save_reg_maybe(result_register)?;

        self.push_type(t1);

        Ok(())
    }

    fn visit_unary_operator(&mut self, v: &mut UnaryOperator) -> Self::Result {
        // Typecheck.
        let t = self.pop_type()?;
        typing::typecheck_unary_operator::<UnaryOperator>(&t)?;

        // Execution.
        let register = self.pop_reg(0)?;
        match v {
            UnaryOperator::Plus => {}
            UnaryOperator::Minus => {
                emit::register_operation("neg", register, &mut self.scopes)?;
                self.save_reg(register)?;
            }
            UnaryOperator::Unknown => panic!("unknown unary operator"),
        }

        self.push_type(t);
        Ok(())
    }

    fn visit_function_declaration(&mut self, v: &mut FunctionDeclaration) -> Self::Result {
        emit::label(&v.name, &mut self.scopes)?;
        v.block.accept(self)?;

        if v.name == "main" {
            emit::syscall(2, &mut self.scopes)?;
        } else {
            emit::ret(&mut self.scopes)?;
        }
        Ok(())
    }

    fn visit_statement(&mut self, v: &mut Statement) -> Self::Result {
        match v {
            Statement::Expr(expr) => expr.accept(self),
            Statement::Return(ret_maybe) => {
                if let Some(ret) = ret_maybe {
                    ret.accept(self)
                } else {
                    Ok(())
                }
            }
            Statement::VarAssign(assignment) => assignment.accept(self),
            Statement::VarDecl(declaration) => declaration.accept(self),
            Statement::IfExpression(if_expr) => if_expr.accept(self),
        }
    }

    fn visit_variable_declaration(&mut self, v: &mut VariableDeclaration) -> Self::Result {
        let variable_type =
            typing::BuiltInType::try_from(v.var_type.clone()).context(UnknownType {
                name: v.var_type.clone(),
            })?;

        let var = self.scopes.current_mut()?.variable_with_size(
            &v.name,
            v.var_type.clone(),
            variable_type.alloc_size(),
        )?;

        if let Some(mut expr) = v.expression.clone() {
            expr.accept(self)?;
            let expr_type = self.pop_type()?;
            ensure!(
                expr_type == var.var_type,
                TypeMismatch {
                    t1: expr_type,
                    t2: v.var_type.clone()
                }
            );
            emit::stack_var_set_sized(var.offset, self.pop_reg(0)?, var.size, &mut self.scopes)?;
        }

        Ok(())
    }

    fn visit_program(&mut self, v: &mut Program) -> Self::Result {
        let mut function_keys: Vec<String> =
            v.functions.iter().map(|(key, _v)| key.clone()).collect();
        function_keys.sort();

        for fn_name in function_keys.into_iter() {
            let decl = v.functions.get_mut(&fn_name).unwrap();
            decl.accept(self)?;
        }

        Ok(())
    }

    fn visit_atomic_expression(&mut self, v: &mut AtomicExpression) -> Self::Result {
        v.atom.accept(self)?;

        // TODO: Visit trailers.

        Ok(())
    }

    fn visit_atom(&mut self, v: &mut Atom) -> Self::Result {
        match v {
            Atom::Boolean(b) => {
                self.type_stack.push(String::from("bool"));
                let bool_val = if *b { 1 } else { 0 };
                self.save_val(bool_val)?;
            }
            Atom::Integer(i) => {
                self.type_stack.push(String::from("int"));
                self.save_val(*i)?;
            }
            Atom::Identifier(i) => {
                let offset = {
                    let var = self.scopes.current()?.get_variable(i.as_ref())?;
                    self.type_stack.push(var.var_type.clone());
                    var.offset
                };

                let result_register = self.get_writeable_register()?;
                emit::stack_offset_load(offset, result_register, &mut self.scopes)?;
                self.save_reg_maybe(result_register)?;
            }
        }
        Ok(())
    }

    fn visit_block(&mut self, v: &mut Block) -> Self::Result {
        self.scopes.push();

        for statement in v.body.iter_mut() {
            statement.accept(self)?;
        }

        emit::scope_declaration(&mut self.scopes)?;
        Ok(())
    }

    fn visit_variable_assignment(&mut self, v: &mut VariableAssignment) -> Self::Result {
        v.expression.accept(self)?;
        let expr_type = self.pop_type()?;

        let reg = self.pop_reg(0)?;

        let var = self.scopes.current_mut()?.get_variable(&v.name)?;
        ensure!(
            expr_type == var.var_type,
            TypeMismatch {
                t1: expr_type,
                t2: var.var_type.clone()
            }
        );
        emit::stack_var_set_sized(var.offset, reg, var.size, &mut self.scopes)?;
        Ok(())
    }

    fn visit_function_call(&mut self, v: &mut FunctionCall) -> Self::Result {
        ensure!(
            self.functions.contains_key(&v.name),
            UnknownFunction {
                name: v.name.clone()
            }
        );
        emit::fn_call(v.name.as_ref(), &mut self.scopes)?;

        // TODO: Get return value,
        Ok(())
    }

    fn visit_if_expression(&mut self, v: &mut IfExpression) -> Self::Result {
        v.condition.accept(self)?;

        // After this, the value of the expression is stored on top of the compiler stack.
        let expr_val_reg = self.pop_reg(0)?;

        let cond_label = "condition";

        self.scopes
            .current_mut()?
            .push_instruction(format!("jez ${} @{}", expr_val_reg, cond_label));

        v.if_block.accept(self)?;

        self.scopes
            .current_mut()?
            .push_instruction(format!("{}:", cond_label));

        if let Some(else_block) = &mut v.else_block {
            else_block.accept(self)?;
        }

        Ok(())
    }
}
