use crate::VM;

#[inline]
pub fn add(reg_src_a: u8, reg_src_b: u8, reg_dst: u8, vm: &mut VM) {
    let res = vm.registers()[reg_src_a as usize] + vm.registers()[reg_src_b as usize];
    log::trace!(
        "add ${}/{:#06x} ${}/{:#06x} => ${}/{:#06x}",
        reg_src_a,
        vm.registers()[reg_src_a as usize],
        reg_src_b,
        vm.registers()[reg_src_b as usize],
        reg_dst,
        res
    );
    vm.registers_mut()[reg_dst as usize] = res;
}

#[inline]
pub fn sub(reg_src_a: u8, reg_src_b: u8, reg_dst: u8, vm: &mut VM) {
    let res = vm.registers()[reg_src_a as usize] - vm.registers()[reg_src_b as usize];
    log::trace!(
        "sub ${}/{:#06x} ${}/{:#06x} => ${}/{:#06x}",
        reg_src_a,
        vm.registers()[reg_src_a as usize],
        reg_src_b,
        vm.registers()[reg_src_b as usize],
        reg_dst,
        res
    );
    vm.registers_mut()[reg_dst as usize] = res;
}

#[inline]
pub fn mul(reg_src_a: u8, reg_src_b: u8, reg_dst: u8, vm: &mut VM) {
    let res = vm.registers()[reg_src_a as usize] * vm.registers()[reg_src_b as usize];
    log::trace!(
        "mul ${}/{:#06x} ${}/{:#06x} => ${}/{:#06x}",
        reg_src_a,
        vm.registers()[reg_src_a as usize],
        reg_src_b,
        vm.registers()[reg_src_b as usize],
        reg_dst,
        res
    );
    vm.registers_mut()[reg_dst as usize] = res;
}

#[inline]
pub fn div(reg_src_a: u8, reg_src_b: u8, reg_dst: u8, vm: &mut VM) {
    let reg_a = reg_src_a as usize;
    let reg_b = reg_src_b as usize;
    vm.registers_mut()[reg_dst as usize] = vm.registers()[reg_a] / vm.registers()[reg_b];
    vm.set_remainder((vm.registers()[reg_a] % vm.registers()[reg_b]) as u32);

    log::trace!(
        "div ${}/{:#06x} ${}/{:#06x} => ${}/{:#06x}r{}",
        reg_a,
        vm.registers()[reg_a],
        reg_b,
        vm.registers()[reg_b],
        reg_dst,
        vm.registers()[reg_dst as usize],
        vm.remainder()
    );
}

#[inline]
pub fn eq(reg_a: u8, reg_b: u8, vm: &mut VM) {
    vm.equal_flag = vm.registers()[reg_a as usize] == vm.registers()[reg_b as usize];

    log::trace!(
        "eq ${}/{:#06x} ${}/{:#06x} => {}",
        reg_a,
        vm.registers()[reg_a as usize],
        reg_b,
        vm.registers()[reg_b as usize],
        vm.equal_flag
    );
}

#[inline]
pub fn neq(reg_a: u8, reg_b: u8, vm: &mut VM) {
    vm.equal_flag = vm.registers()[reg_a as usize] != vm.registers()[reg_b as usize];

    log::trace!(
        "neq ${}/{:#06x} ${}/{:#06x} => {}",
        reg_a,
        vm.registers()[reg_a as usize],
        reg_b,
        vm.registers()[reg_b as usize],
        vm.equal_flag
    );
}

#[inline]
pub fn gt(reg_1: u8, reg_2: u8, vm: &mut VM) {
    vm.equal_flag = vm.registers()[reg_1 as usize] > vm.registers()[reg_2 as usize];

    log::trace!(
        "gt ${}/{:#06x} ${}/{:#06x} => {}",
        reg_1,
        vm.registers()[reg_1 as usize],
        reg_2,
        vm.registers()[reg_2 as usize],
        vm.equal_flag
    );
}

#[inline]
pub fn lt(reg_1: u8, reg_2: u8, vm: &mut VM) {
    vm.equal_flag = vm.registers()[reg_1 as usize] < vm.registers()[reg_2 as usize];

    log::trace!(
        "lt ${}/{:#06x} ${}/{:#06x} => {}",
        reg_1,
        vm.registers()[reg_1 as usize],
        reg_2,
        vm.registers()[reg_2 as usize],
        vm.equal_flag
    );
}

#[inline]
pub fn gtq(reg_1: u8, reg_2: u8, vm: &mut VM) {
    vm.equal_flag = vm.registers()[reg_1 as usize] >= vm.registers()[reg_2 as usize];

    log::trace!(
        "gtq ${}/{:#06x} ${}/{:#06x} => {}",
        reg_1,
        vm.registers()[reg_1 as usize],
        reg_2,
        vm.registers()[reg_2 as usize],
        vm.equal_flag
    );
}

#[inline]
pub fn ltq(reg_1: u8, reg_2: u8, vm: &mut VM) {
    vm.equal_flag = vm.registers()[reg_1 as usize] <= vm.registers()[reg_2 as usize];

    log::trace!(
        "ltq ${}/{:#06x} ${}/{:#06x} => {}",
        reg_1,
        vm.registers()[reg_1 as usize],
        reg_2,
        vm.registers()[reg_2 as usize],
        vm.equal_flag
    );
}
