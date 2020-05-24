use std::mem;

use instructor::{STACK_BASE_REGISTER, STACK_POINTER_REGISTER};

use crate::VM;

#[inline]
pub fn push(register: u8, vm: &mut VM) {
    let value = vm.registers()[register as usize];

    log::trace!("push ${}/{:#06x}", register, value);

    vm.stack_mut().push_i32(value);

    vm.registers_mut()[STACK_POINTER_REGISTER] += std::mem::size_of::<i32>() as i32;
}

#[inline]
pub fn pop(register: u8, vm: &mut VM) {
    if vm.stack().len() != 0 {
        vm.registers_mut()[STACK_POINTER_REGISTER] -= mem::size_of::<i32>() as i32;
        debug_assert!(vm.registers()[STACK_POINTER_REGISTER] > 0);
    }

    let value = vm.stack_mut().pop_i32();

    log::trace!("pop ${}/{:#06x}", register, value);
    vm.registers_mut()[register as usize] = value;
}
