use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use instructor::{Address, MemorySection};

use crate::{memutil, op::memory, VM};

#[test]
fn op_sw() {
    // Typical case of store-word.
    let mut vm = VM::new();
    vm.heap_mut().alloc(4);
    vm.registers_mut()[0] = 42;
    vm.registers_mut()[1] = 0;
    memory::sw(0, &Address::new_heap(1, 0), &mut vm);

    assert_eq!(&vm.registers()[0..2], vec![42, 0].as_slice());
    assert_eq!(vm.heap().memory().read_i32::<LittleEndian>().unwrap(), 42);
}

#[test]
#[should_panic(expected = "out of range")]
fn op_sw_invalid_ptr() {
    let mut vm = VM::new();
    vm.heap_mut().alloc(4);
    vm.registers_mut()[0] = 42;
    vm.registers_mut()[1] = 15;
    memory::sw(0, &Address::new_heap(1, 0), &mut vm);
}

#[test]
fn op_sw_offset() {
    // Make sure manual ptr offsets work.
    let mut vm = VM::new();
    vm.heap_mut().alloc(8);
    vm.registers_mut()[0] = 42;
    vm.registers_mut()[1] = 2;
    memory::sw(0, &Address::new_heap(1, 2), &mut vm);

    assert_eq!(&vm.registers()[0..2], vec![42, 2].as_slice());
    assert_eq!(&vm.heap().memory()[0..4], vec![0; 4].as_slice());
    assert_eq!(
        (&vm.heap().memory()[4..])
            .read_i32::<LittleEndian>()
            .unwrap(),
        42
    );
}

#[test]
#[should_panic(expected = "out of range")]
fn op_sw_memory_too_small() {
    // Typical case of store-word.
    let mut vm = VM::new();
    vm.heap_mut().alloc(2);
    vm.registers_mut()[0] = 42;
    vm.registers_mut()[1] = (memutil::WORD_WIDTH - 2) as i32;
    memory::sw(0, &Address::new_heap(1, 0), &mut vm);
}

#[test]
fn op_sb() {
    // Typical case of store-word.
    let mut vm = VM::new();
    vm.heap_mut().alloc(4);
    vm.registers_mut()[0] = 42;
    vm.registers_mut()[1] = 0;
    memory::sb(0, &Address::new_heap(1, 0), &mut vm);

    assert_eq!(&vm.registers()[0..2], vec![42, 0].as_slice());
    assert_eq!(&vm.heap().memory()[0..4], vec![42, 0, 0, 0].as_slice());
}

#[test]
#[should_panic(expected = "out of bounds")]
fn op_sb_invalid_ptr() {
    let mut vm = VM::new();
    vm.heap_mut().alloc(4);
    vm.registers_mut()[0] = 42;
    vm.registers_mut()[1] = 15;
    memory::sb(0, &Address::new_heap(1, 0), &mut vm);
}

#[test]
fn op_sb_offset() {
    // Make sure manual ptr offsets work.
    let mut vm = VM::new();
    vm.heap_mut().alloc(8);
    vm.registers_mut()[0] = 42;
    vm.registers_mut()[1] = 2;
    memory::sb(0, &Address::new_heap(1, 2), &mut vm);

    assert_eq!(&vm.registers()[0..2], vec![42, 2].as_slice());
    assert_eq!(&vm.heap().memory()[0..4], vec![0; 4].as_slice());
    assert_eq!(
        (&vm.heap().memory()[4..])
            .read_i32::<LittleEndian>()
            .unwrap(),
        42
    );
}

#[test]
fn op_lw() {
    let mut vm = VM::new();
    vm.heap_mut().alloc(8);
    vm.registers_mut()[0] = 0;
    vm.registers_mut()[1] = 4;
    (&mut vm.heap_mut().memory_mut()[4..8])
        .write_i32::<LittleEndian>(45)
        .unwrap();
    memory::lw(0, &Address::new_heap(1, 0), &mut vm);
    assert_eq!(vm.registers()[0], 45);
}

#[test]
fn op_lw_offset() {
    let mut vm = VM::new();
    vm.heap_mut().alloc(8);
    vm.registers_mut()[0] = 0;
    vm.registers_mut()[1] = 2;
    (&mut vm.heap_mut().memory_mut()[4..8])
        .write_i32::<LittleEndian>(45)
        .unwrap();
    memory::lw(0, &Address::new_heap(1, 2), &mut vm);
    assert_eq!(vm.registers()[0], 45);
}

#[test]
#[should_panic(expected = "out of range")]
fn op_lw_invalid_ptr() {
    let mut vm = VM::new();
    vm.heap_mut().alloc(4);
    vm.registers_mut()[0] = 0;
    vm.registers_mut()[1] = 18;
    memory::lw(0, &Address::new_heap(1, 0), &mut vm);
}

#[test]
#[should_panic(expected = "out of range")]
fn op_lw_memory_too_small() {
    let mut vm = VM::new();
    vm.heap_mut().alloc(2);
    vm.registers_mut()[0] = 0;
    vm.registers_mut()[1] = (memutil::WORD_WIDTH - 2) as i32;
    memory::lw(0, &Address::new_heap(1, 0), &mut vm);
}

#[test]
fn op_lb() {
    let mut vm = VM::new();
    vm.heap_mut().alloc(4);
    vm.registers_mut()[0] = 0;
    vm.registers_mut()[1] = 3;
    vm.heap_mut().memory_mut()[3] = 18;
    memory::lb(0, &Address::new_heap(1, 0), &mut vm);
    assert_eq!(vm.registers()[0], 18);
}

#[test]
fn op_lb_offset() {
    let mut vm = VM::new();
    vm.heap_mut().alloc(8);
    vm.registers_mut()[0] = 0;
    vm.registers_mut()[1] = 2;
    vm.heap_mut().memory_mut()[5] = 14;
    memory::lb(0, &Address::new_heap(1, 3), &mut vm);
    assert_eq!(vm.registers()[0], 14);
}

#[test]
#[should_panic(expected = "out of bounds")]
fn op_lb_invalid_ptr() {
    let mut vm = VM::new();
    vm.heap_mut().alloc(4);
    vm.registers_mut()[0] = 0;
    vm.registers_mut()[1] = 18;
    memory::lb(0, &Address::new_heap(1, 0), &mut vm);
}
