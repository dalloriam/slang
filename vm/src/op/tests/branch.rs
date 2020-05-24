use crate::op::branch;
use crate::VM;

#[test]
fn op_jmp() {
    let mut vm = VM::new();
    assert_eq!(vm.pc, 0);

    branch::jmp(10, &mut vm);

    assert_eq!(vm.pc, 10);
}

#[test]
fn op_jmpf() {
    let mut vm = VM::new();
    vm.pc = 5;
    branch::jmpf(5, &mut vm);
    assert_eq!(vm.pc, 10);
}

#[test]
fn op_jmpb() {
    let mut vm = VM::new();
    vm.pc = 5;
    branch::jmpb(5, &mut vm);
    assert_eq!(vm.pc, 0);
}

#[test]
fn op_rjmp() {
    let mut vm = VM::new();
    vm.registers_mut()[10] = 5;
    vm.pc = 10;

    branch::rjmp(10, &mut vm);
    assert_eq!(vm.pc, 5);
}

#[test]
fn op_jeq_true() {
    let mut vm = VM::new();
    vm.pc = 5;
    vm.equal_flag = true;

    branch::jeq(10, &mut vm);
    assert_eq!(vm.pc, 10);
}

#[test]
fn op_jeq_false() {
    let mut vm = VM::new();
    vm.pc = 5;
    vm.equal_flag = false;

    branch::jeq(10, &mut vm);
    assert_eq!(vm.pc, 5);
}
