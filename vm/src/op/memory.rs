use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::VM;

#[inline]
pub fn sw(src_reg: u8, ptr_reg: u8, ptr_offset: u8, vm: &mut VM) {
    let ptr = vm.registers()[ptr_reg as usize] as usize + ptr_offset as usize;

    let value_to_write = vm.registers()[src_reg as usize];

    log::trace!("sw ${}/{:#06x} => @{:#06x}", src_reg, value_to_write, ptr);

    let mut slice = &mut vm.heap_mut().memory_mut()[ptr..ptr + 4];
    slice.write_i32::<LittleEndian>(value_to_write).unwrap(); // TODO: Catch error.
}

#[inline]
pub fn lw(dst_reg: u8, ptr_reg: u8, ptr_offset: u8, vm: &mut VM) {
    let ptr = vm.registers()[ptr_reg as usize] as usize + ptr_offset as usize;

    let mut slice = &vm.heap().memory()[ptr..ptr + 4];

    // TODO: Catch error
    vm.registers_mut()[dst_reg as usize] = slice.read_i32::<LittleEndian>().unwrap();

    log::trace!(
        "lw @{:#06x} => ${}/{:#06x}",
        ptr,
        dst_reg,
        vm.registers()[dst_reg as usize]
    );
}

#[inline]
pub fn sb(src_reg: u8, ptr_reg: u8, ptr_offset: u8, vm: &mut VM) {
    // Set byte.
    let value_to_write = vm.registers()[src_reg as usize] as u8; // TODO: Ensure < 256

    let ptr = vm.registers()[ptr_reg as usize] as usize + ptr_offset as usize;

    log::trace!("sb ${}/{:#04x} => @{:#06x}", src_reg, value_to_write, ptr);
    vm.heap_mut().memory_mut()[ptr] = value_to_write;
}

#[inline]
pub fn lb(dst_reg: u8, ptr_reg: u8, ptr_offset: u8, vm: &mut VM) {
    let ptr = vm.registers()[ptr_reg as usize] as usize + ptr_offset as usize;

    let val = vm.heap().memory()[ptr];
    vm.registers_mut()[dst_reg as usize] = val as i32;

    log::trace!("lb @{:#06x} => #{}/{:#06x}", ptr, dst_reg, val);
}
