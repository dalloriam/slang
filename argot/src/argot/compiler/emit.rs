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

pub fn stack_offset_load_word(offset: i32, register: u8, scopes: &mut ScopeManager) -> Result<()> {
    let scope = scopes.current_mut()?;
    scope.push_instruction(format!("lw ${} {}[$ebp]", register, offset));
    Ok(())
}
pub fn stack_offset_load_byte(offset: i32, register: u8, scopes: &mut ScopeManager) -> Result<()> {
    let scope = scopes.current_mut()?;
    scope.push_instruction(format!("lb ${} {}[$ebp]", register, offset));
    Ok(())
}

pub fn stack_var_load_sized(
    offset: i32,
    register: u8,
    size: usize,
    scopes: &mut ScopeManager,
) -> Result<()> {
    if size == 4 {
        stack_offset_load_word(offset, register, scopes)
    } else if size == 1 {
        stack_offset_load_byte(offset, register, scopes)
    } else {
        panic!("Bad alloc size")
    }
}

pub fn stack_offset_set_word(offset: i32, register: u8, scopes: &mut ScopeManager) -> Result<()> {
    scopes
        .current_mut()?
        .push_instruction(format!("sw ${} {}[$ebp]", register, offset));
    Ok(())
}

pub fn stack_offset_set_byte(offset: i32, register: u8, scopes: &mut ScopeManager) -> Result<()> {
    scopes
        .current_mut()?
        .push_instruction(format!("sb ${} {}[$ebp]", register, offset));
    Ok(())
}

pub fn stack_var_set_sized(
    offset: i32,
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

pub fn stack_push_sized(register: u8, size: usize, scopes: &mut ScopeManager) -> Result<()> {
    if size == 4 {
        scopes
            .current_mut()?
            .push_instruction(format!("pushw ${}", register));
    } else if size == 1 {
        scopes
            .current_mut()?
            .push_instruction(format!("pushb ${}", register));
    } else {
        panic!("Bad alloc size")
    }

    Ok(())
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

pub fn inline_binary_op(
    operation: &str,
    operand_reg_1: u8,
    operand_reg_2: u8,
    scopes: &mut ScopeManager,
) -> Result<()> {
    scopes.current_mut()?.push_instruction(format!(
        "{} ${} ${}",
        operation, operand_reg_1, operand_reg_2
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

pub fn jump_to_label(label: &str, scopes: &mut ScopeManager) -> Result<()> {
    scopes
        .current_mut()?
        .push_instruction(format!("jmp @{}", label));
    Ok(())
}

pub fn jump_to_else(
    value_register: u8,
    condition_label: &str,
    scopes: &mut ScopeManager,
) -> Result<()> {
    scopes
        .current_mut()?
        .push_instruction(format!("jez ${} @{}", value_register, condition_label));

    Ok(())
}

pub fn rshift(val_register: u8, amt_register: u8, scopes: &mut ScopeManager) -> Result<()> {
    scopes
        .current_mut()?
        .push_instruction(format!("shr ${} ${}", val_register, amt_register));
    Ok(())
}

pub fn bitwise_not(register: u8, scopes: &mut ScopeManager) -> Result<()> {
    scopes
        .current_mut()?
        .push_instruction(format!("not ${}", register));
    Ok(())
}

pub fn negation(value_register: u8, scopes: &mut ScopeManager) -> Result<()> {
    // Logical negation of 32-bit signed integers using bitwise operators:
    // (!(x >> 1) + x) >> 31

    let swap_register = 5;
    let one_register = 1;
    let thirty_one = 2;

    // Load our shift amounts in temporary registers.
    save_to_register(1, one_register, scopes)?;
    save_to_register(31, thirty_one, scopes)?;

    // Insert x in swap_register to get a copy
    mov(value_register, swap_register, scopes)?;

    // Perform the first right shift by one.
    rshift(value_register, one_register, scopes)?;

    // Negate the result.
    bitwise_not(value_register, scopes)?;

    // Add back x.
    binary_operation("add", value_register, swap_register, value_register, scopes)?;

    // Shift the result by 31.
    rshift(value_register, thirty_one, scopes)?;

    Ok(())
}
