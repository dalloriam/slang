use std::mem;

use instructor::{STACK_BASE_REGISTER, STACK_POINTER_REGISTER};

use crate::VM;

#[inline]
pub fn jmp(target_addr: u16, vm: &mut VM) {
    vm.pc = target_addr as usize;
    log::trace!("jmp {:#06x}", target_addr);
}

#[inline]
pub fn jmpf(addr_offset: u16, vm: &mut VM) {
    vm.pc += addr_offset as usize;
    log::trace!("jmpf {:#06x}", addr_offset);
}

#[inline]
pub fn jmpb(addr_offset: u16, vm: &mut VM) {
    vm.pc -= addr_offset as usize;
    log::trace!("jmpb {:#06x}", addr_offset);
}

#[inline]
pub fn rjmp(register: u8, vm: &mut VM) {
    let value = vm.registers()[register as usize];
    vm.pc = value as usize;

    log::trace!("rjmp ${}/{:#06x}", register, value);
}

#[inline]
pub fn jeq(target_addr: u16, vm: &mut VM) {
    if vm.equal_flag {
        vm.pc = target_addr as usize;
    }

    log::trace!("jeq {:#06x} => {}", target_addr, vm.equal_flag);
}

#[inline]
pub fn call(target_addr: u16, vm: &mut VM) {
    log::trace!("call {:#06x}", target_addr);

    // Push the return address (which is the current pc) on the stack.
    // Function arguments must be pushed on the stack before calling call().
    let return_address = vm.pc;
    log::trace!("PUSHING RETURN ADDRESS: {:#06x}", return_address);
    vm.stack_mut().push_i32(return_address as i32);
    vm.registers_mut()[STACK_POINTER_REGISTER] += mem::size_of::<i32>() as i32;

    // Jump to the beginning of the function.
    vm.pc = target_addr as usize;

    // Save the pre-call ebp (current base pointer on the stack).
    let current_stack_base = vm.registers()[STACK_BASE_REGISTER];
    vm.stack_mut().push_i32(current_stack_base);
    vm.registers_mut()[STACK_POINTER_REGISTER] += mem::size_of::<i32>() as i32;

    // Overwrite the ebp with the current value of the esp.
    vm.registers_mut()[STACK_BASE_REGISTER] = vm.registers()[STACK_POINTER_REGISTER];
}

#[inline]
pub fn ret(vm: &mut VM) {
    log::trace!("ret");
    // We assume here that the return value of the function is stored in $0.

    // Pop the saved ebp from the stack - This tears down the stack frame.
    let old_stack_base = vm.stack_mut().pop_i32(); // TODO: Safety
    vm.registers_mut()[STACK_BASE_REGISTER] = old_stack_base;
    vm.registers_mut()[STACK_POINTER_REGISTER] -= mem::size_of::<i32>() as i32;

    let return_address = vm.stack_mut().pop_i32() as usize;
    vm.registers_mut()[STACK_POINTER_REGISTER] -= mem::size_of::<i32>() as i32;
    log::trace!("LOADED RETURN ADDRESS: {:#06x}", return_address);
    vm.pc = return_address;
}
