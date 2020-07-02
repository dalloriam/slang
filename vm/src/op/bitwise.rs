use crate::VM;

#[inline]
pub fn not(register: u8, vm: &mut VM) {
    let val = vm.registers()[register as usize];
    vm.registers_mut()[register as usize] = !val;
    log::trace!("not ${}/{:#06x} => {:#06x}", register, val, !val);
}

#[inline]
pub fn shiftl(register_dst: u8, register_src: u8, vm: &mut VM) {
    let val = vm.registers()[register_dst as usize];
    let shift_amt = vm.registers()[register_src as usize];
    vm.registers_mut()[register_dst as usize] = val.checked_shl(shift_amt as u32).unwrap_or(0);
    log::trace!(
        "shl ${}/{:#06x} << ${}/{:#06x} -> {:#06x}",
        register_dst,
        val,
        register_src,
        shift_amt,
        val.checked_shl(shift_amt as u32).unwrap_or(0)
    );
}

#[inline]
pub fn shiftr(register_dst: u8, register_src: u8, vm: &mut VM) {
    let val = vm.registers()[register_dst as usize];
    let shift_amt = vm.registers()[register_src as usize];
    vm.registers_mut()[register_dst as usize] = val.checked_shr(shift_amt as u32).unwrap_or(0);
    log::trace!(
        "shr ${}/{:#06x} >> ${}/{:#06x} -> {:#06x}",
        register_dst,
        val,
        register_src,
        shift_amt,
        val.checked_shr(shift_amt as u32).unwrap_or(0)
    );
}

#[inline]
pub fn and(register_dst: u8, register_src: u8, vm: &mut VM) {
    let src_val = vm.registers()[register_src as usize];
    let dst_val = vm.registers()[register_dst as usize];
    vm.registers_mut()[register_dst as usize] = src_val & dst_val;
    log::trace!(
        "and ${}/{:#06x} ${}/{:#06x} -> {:#06x}",
        register_src,
        src_val,
        register_dst,
        dst_val,
        src_val & dst_val
    );
}

#[inline]
pub fn or(register_dst: u8, register_src: u8, vm: &mut VM) {
    let src_val = vm.registers()[register_src as usize];
    let dst_val = vm.registers()[register_dst as usize];
    vm.registers_mut()[register_dst as usize] = src_val | dst_val;
    log::trace!(
        "or ${}/{:#06x} ${}/{:#06x} -> {:#06x}",
        register_src,
        src_val,
        register_dst,
        dst_val,
        src_val | dst_val
    );
}
