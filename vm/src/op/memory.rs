use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use instructor::{Address, MemorySection, STACK_POINTER_REGISTER};

use crate::VM;

#[inline]
pub fn sw(src_reg: u8, addr: &Address, vm: &mut VM) {
    let ptr = vm.registers()[addr.register as usize] as usize + addr.offset as usize;

    let value_to_write = vm.registers()[src_reg as usize];

    log::trace!("sw ${}/{:#06x} => @{:#06x}", src_reg, value_to_write, ptr);

    let mut memory_slice = match addr.section {
        MemorySection::Heap => &mut vm.heap_mut().memory_mut()[ptr..ptr + 4],
        MemorySection::Stack => {
            // This can cause stack growth, need to update the esp
            vm.stack_mut().safe_grow(ptr + 4);

            vm.registers_mut()[STACK_POINTER_REGISTER] = vm.stack().len() as i32;

            &mut vm.stack_mut().memory_mut()[ptr..ptr + 4]
        }
    };

    memory_slice
        .write_i32::<LittleEndian>(value_to_write)
        .unwrap(); // TODO: Catch error.
}

#[inline]
pub fn lw(dst_reg: u8, addr: &Address, vm: &mut VM) {
    let ptr = vm.registers()[addr.register as usize] as usize + addr.offset as usize;

    let mut memory_slice = match addr.section {
        MemorySection::Heap => &vm.heap().memory()[ptr..ptr + 4],
        MemorySection::Stack => &vm.stack().memory()[ptr..ptr + 4],
    };

    // TODO: Catch error
    vm.registers_mut()[dst_reg as usize] = memory_slice.read_i32::<LittleEndian>().unwrap();

    log::trace!(
        "lw @{:#06x} => ${}/{:#06x}",
        ptr,
        dst_reg,
        vm.registers()[dst_reg as usize]
    );
}

#[inline]
pub fn sb(src_reg: u8, addr: &Address, vm: &mut VM) {
    // Set byte.
    let value_to_write = vm.registers()[src_reg as usize] as u8; // TODO: Ensure < 256

    let ptr = vm.registers()[addr.register as usize] as usize + addr.offset as usize;

    log::trace!("sb ${}/{:#04x} => @{:#06x}", src_reg, value_to_write, ptr);

    match addr.section {
        MemorySection::Heap => vm.heap_mut().memory_mut()[ptr] = value_to_write,
        MemorySection::Stack => vm.stack_mut().memory_mut()[ptr] = value_to_write,
    }
}

#[inline]
pub fn lb(dst_reg: u8, addr: &Address, vm: &mut VM) {
    let ptr = vm.registers()[addr.register as usize] as usize + addr.offset as usize;

    let val = match addr.section {
        MemorySection::Heap => vm.heap().memory()[ptr],
        MemorySection::Stack => vm.stack().memory()[ptr],
    };
    vm.registers_mut()[dst_reg as usize] = val as i32;

    log::trace!("lb @{:#06x} => #{}/{:#06x}", ptr, dst_reg, val);
}
