use crate::VM;

#[inline]
pub fn load(register: u8, value: u16, vm: &mut VM) {
    vm.registers_mut()[register as usize] = value as i32;
    log::trace!("ld {:#06x} => ${}", value, register);
}

#[inline]
pub fn inc(register: u8, vm: &mut VM) {
    vm.registers_mut()[register as usize] += 1;
    log::trace!("inc ${}/{}", register, vm.registers()[register as usize]);
}

#[inline]
pub fn dec(register: u8, vm: &mut VM) {
    vm.registers_mut()[register as usize] -= 1;
    log::trace!("dec ${}/{}", register, vm.registers()[register as usize]);
}

#[inline]
pub fn mov(from_reg: u8, to_reg: u8, vm: &mut VM) {
    log::trace!(
        "mov ${}/{:#06x} => ${}",
        from_reg,
        vm.registers()[from_reg as usize],
        to_reg
    );

    vm.registers_mut()[to_reg as usize] = vm.registers()[from_reg as usize];
}
