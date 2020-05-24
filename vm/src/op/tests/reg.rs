use crate::op::reg;
use crate::VM;

#[test]
fn op_load() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 3;
    reg::load(0, 5, &mut vm);
    assert_eq!(vm.registers()[0], 5);
}

#[test]
#[should_panic(expected = "out of bounds")]
fn op_load_invalid_register() {
    let mut vm = VM::new();
    reg::load(45, 1, &mut vm);
}

#[test]
fn op_inc() {
    let mut vm = VM::new();
    vm.registers_mut()[1] = 14;
    reg::inc(1, &mut vm);
    assert_eq!(vm.registers()[1], 15);
}

#[test]
fn op_dec() {
    let mut vm = VM::new();
    vm.registers_mut()[1] = 15;
    reg::dec(1, &mut vm);
    assert_eq!(vm.registers()[1], 14);
}

#[test]
fn op_mov() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 4;
    vm.registers_mut()[1] = 0;
    reg::mov(0, 1, &mut vm);
    assert_eq!(&vm.registers()[0..2], vec![4, 4].as_slice());
}

#[test]
fn op_mov_same_register() {
    // TODO: Optimize out movs to same reg.
    let mut vm = VM::new();
    vm.registers_mut()[0] = 4;
    vm.registers_mut()[1] = 0;
    reg::mov(0, 0, &mut vm);
    assert_eq!(&vm.registers()[0..2], vec![4, 0].as_slice());
}
