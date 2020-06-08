use std::convert::TryFrom;

use snafu::ResultExt;

use crate::compiler::{error::*, scope::ScopeManager, typing};

pub fn save_to_register(value_to_save: i32, register: u8, scopes: &mut ScopeManager) -> Result<()> {
    scopes
        .current_mut()?
        .push_instruction(format!("ld ${} {:#06x}", register, value_to_save));
    Ok(())
}

pub fn stack_push_word(register: u8, scopes: &mut ScopeManager) -> Result<()> {
    scopes
        .current_mut()?
        .push_instruction(format!("pushw ${}", register));
    Ok(())
}

pub fn stack_pop_word(register: u8, scopes: &mut ScopeManager) -> Result<()> {
    scopes
        .current_mut()?
        .push_instruction(format!("popw ${}", register));
    Ok(())
}

pub fn stack_pop_byte(register: u8, scopes: &mut ScopeManager) -> Result<()> {
    scopes
        .current_mut()?
        .push_instruction(format!("popb ${}", register));
    Ok(())
}

pub fn stack_pop_sized(register: u8, size: usize, scopes: &mut ScopeManager) -> Result<()> {
    if size == 4 {
        stack_pop_word(register, scopes)
    } else if size == 1 {
        stack_pop_byte(register, scopes)
    } else {
        panic!("Bad alloc size")
    }
}

pub fn mov(src_reg: u8, dst_reg: u8, scopes: &mut ScopeManager) -> Result<()> {
    scopes
        .current_mut()?
        .push_instruction(format!("move ${} ${}", src_reg, dst_reg));
    Ok(())
}

pub fn stack_offset_load(offset: usize, register: u8, scopes: &mut ScopeManager) -> Result<()> {
    let scope = scopes.current_mut()?;
    scope.push_instruction(format!("lw ${} {}[$ebp]", register, offset));
    Ok(())
}

pub fn stack_offset_set_word(offset: usize, register: u8, scopes: &mut ScopeManager) -> Result<()> {
    scopes
        .current_mut()?
        .push_instruction(format!("sw ${} {}[$ebp]", register, offset));
    Ok(())
}

pub fn stack_offset_set_byte(offset: usize, register: u8, scopes: &mut ScopeManager) -> Result<()> {
    scopes
        .current_mut()?
        .push_instruction(format!("sb ${} {}[$ebp]", register, offset));
    Ok(())
}

pub fn stack_var_set_sized(
    offset: usize,
    register: u8,
    size: usize,
    scopes: &mut ScopeManager,
) -> Result<()> {
    if size == 4 {
        stack_offset_set_word(offset, register, scopes)
    } else if size == 1 {
        stack_offset_set_byte(offset, register, scopes)
    } else {
        panic!("Bad alloc size")
    }
}

pub fn binary_operation(
    operation: &str,
    operand_reg_1: u8,
    operand_reg_2: u8,
    result_reg: u8,
    scopes: &mut ScopeManager,
) -> Result<()> {
    scopes.current_mut()?.push_instruction(format!(
        "{} ${} ${} ${}",
        operation, operand_reg_1, operand_reg_2, result_reg
    ));
    Ok(())
}

pub fn register_operation(operation: &str, register: u8, scopes: &mut ScopeManager) -> Result<()> {
    scopes
        .current_mut()?
        .push_instruction(format!("{} ${}", operation, register));
    Ok(())
}

pub fn label(label: &str, scopes: &mut ScopeManager) -> Result<()> {
    scopes
        .current_mut()?
        .push_instruction(format!("{}:", label));
    Ok(())
}

pub fn syscall(syscall_id: u16, scopes: &mut ScopeManager) -> Result<()> {
    scopes
        .current_mut()?
        .push_instruction(format!("ld $v0 {:#06x}", syscall_id));

    scopes
        .current_mut()?
        .push_instruction(String::from("syscall"));
    Ok(())
}

pub fn ret(scopes: &mut ScopeManager) -> Result<()> {
    scopes.current_mut()?.push_instruction(String::from("ret"));
    Ok(())
}

pub fn scope_declaration(scopes: &mut ScopeManager) -> Result<()> {
    let mut last_scope = scopes.pop()?;
    for var in last_scope.sorted_variables().into_iter() {
        let var_type =
            typing::BuiltInType::try_from(var.var_type.clone()).context(UnknownType {
                name: var.var_type.clone(),
            })?;
        stack_var_set_sized(var.offset, 0, var_type.alloc_size(), scopes)?;
    }

    scopes.current_mut()?.extend(&mut last_scope);

    for var in last_scope.sorted_variables().into_iter().rev() {
        let var_type =
            typing::BuiltInType::try_from(var.var_type.clone()).context(UnknownType {
                name: var.var_type.clone(),
            })?;
        stack_pop_sized(0, var_type.alloc_size(), scopes)?;
    }

    Ok(())
}

pub fn fn_call(fn_name: &str, scopes: &mut ScopeManager) -> Result<()> {
    scopes
        .current_mut()?
        .push_instruction(format!("call @{}", fn_name));
    Ok(())
}

pub fn if_condition() {}
