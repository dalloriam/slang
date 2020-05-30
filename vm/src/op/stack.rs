use std::mem;

use instructor::STACK_POINTER_REGISTER;

use crate::VM;

#[inline]
pub fn pushw(register: u8, vm: &mut VM) {
    let value = vm.registers()[register as usize];

    log::trace!("pushw ${}/{:#06x}", register, value);

    vm.stack_mut().push_i32(value);

    vm.registers_mut()[STACK_POINTER_REGISTER] += std::mem::size_of::<i32>() as i32;
}

#[inline]
pub fn popw(register: u8, vm: &mut VM) {
    if !vm.stack().is_empty() {
        vm.registers_mut()[STACK_POINTER_REGISTER] -= mem::size_of::<i32>() as i32;
        debug_assert!(vm.registers()[STACK_POINTER_REGISTER] >= 0);
    }

    let value = vm.stack_mut().pop_i32();

    log::trace!("popw ${}/{:#06x}", register, value);
    vm.registers_mut()[register as usize] = value;
}

#[inline]
pub fn pushb(register: u8, vm: &mut VM) {
    let value = vm.registers()[register as usize] as u8;

    log::trace!("pushb ${}/{:#04x}", register, value);

    vm.stack_mut().push_u8(value);
    vm.registers_mut()[STACK_POINTER_REGISTER] += 1;
}

#[inline]
pub fn popb(register: u8, vm: &mut VM) {
    if !vm.stack().is_empty() {
        vm.registers_mut()[STACK_POINTER_REGISTER] -= 1;
        debug_assert!(vm.registers()[STACK_POINTER_REGISTER] >= 0);
    }

    let value = vm.stack_mut().pop_u8();

    log::trace!("popb ${}/{:#04x}", register, value);
    vm.registers_mut()[register as usize] = value as i32;
}
