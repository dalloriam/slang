use crate::op::ro;
use crate::VM;

#[test]
pub fn op_lcw() {
    let mut vm = VM::with_ro_block(vec![0, 0, 0, 0, 0, 2, 0, 0]);
    ro::lcw(0, 4, &mut vm);
    assert_eq!(vm.registers()[0], 512)
}

#[test]
#[should_panic(expected = "out of range")]
pub fn op_lcw_bad_offset() {
    let mut vm = VM::with_ro_block(vec![0, 0, 0, 0]);
    ro::lcw(0, 10, &mut vm);
}

#[test]
#[should_panic(expected = "out of range")]
pub fn op_lcw_not_enough_space() {
    let mut vm = VM::with_ro_block(vec![0, 0]);
    ro::lcw(0, 0, &mut vm);
}
