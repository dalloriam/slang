use std::mem;

use crate::op::bitwise;
use crate::VM;

#[test]
fn op_not() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 18;
    bitwise::not(0, &mut vm);

    assert_eq!(vm.registers()[0], !18);
}

#[test]
fn op_shiftl() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 21;
    vm.registers_mut()[1] = 1;
    bitwise::shiftl(0, 1, &mut vm);

    assert_eq!(vm.registers()[1], 1);
    assert_eq!(vm.registers()[0], 42)
}

#[test]
fn op_shiftl_overflow() {
    const OVERFLOW_SHIFT_AMT: i32 = 8 * mem::size_of::<i32>() as i32;

    let mut vm = VM::new();
    vm.registers_mut()[0] = 0x00000001;
    vm.registers_mut()[1] = OVERFLOW_SHIFT_AMT;
    bitwise::shiftl(0, 1, &mut vm);

    assert_eq!(vm.registers()[1], OVERFLOW_SHIFT_AMT);
    assert_eq!(vm.registers()[0], 0);
}

#[test]
fn op_shiftr() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 84;
    vm.registers_mut()[1] = 1;
    bitwise::shiftr(0, 1, &mut vm);

    assert_eq!(vm.registers()[1], 1);
    assert_eq!(vm.registers()[0], 42)
}

#[test]
fn op_shiftr_overflow() {
    const OVERFLOW_SHIFT_AMT: i32 = 8 * mem::size_of::<i32>() as i32;

    let mut vm = VM::new();

    vm.registers_mut()[0] = 0x0f000000;
    vm.registers_mut()[1] = OVERFLOW_SHIFT_AMT;

    bitwise::shiftr(0, 1, &mut vm);

    assert_eq!(vm.registers()[1], OVERFLOW_SHIFT_AMT);
    assert_eq!(vm.registers()[0], 0);
}

#[test]
fn op_and() {
    // 42: 101010
    // 46: 101110
    // 59: 111011
    let mut vm = VM::new();
    vm.registers_mut()[0] = 46;
    vm.registers_mut()[1] = 59;
    bitwise::and(1, 0, &mut vm);

    assert_eq!(vm.registers()[0], 46);
    assert_eq!(vm.registers()[1], 42);
}

#[test]
fn op_or() {
    // 34: 100010
    // 8:  001000
    let mut vm = VM::new();
    vm.registers_mut()[0] = 34;
    vm.registers_mut()[1] = 8;
    bitwise::or(1, 0, &mut vm);

    assert_eq!(vm.registers()[0], 34);
    assert_eq!(vm.registers()[1], 42);
}
