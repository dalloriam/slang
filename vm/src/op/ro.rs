use byteorder::{LittleEndian, ReadBytesExt};

use crate::VM;

#[inline]
pub fn lcw(register: u8, ro_offset: u16, vm: &mut VM) {
    let offset = ro_offset as usize;
    let val = (&vm.ro_block()[offset..offset + 4])
        .read_i32::<LittleEndian>()
        .unwrap();

    log::trace!("lcw @{:#06x}/{:#06x} => ${}", offset, val, register);

    vm.registers_mut()[register as usize] = val;
}
