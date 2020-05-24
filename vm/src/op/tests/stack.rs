use crate::op::stack;
use crate::VM;
use instructor::STACK_POINTER_REGISTER;

#[test]
fn op_push() {
    let mut vm = VM::new();
    vm.registers_mut()[1] = 18;
    assert!(vm.stack().is_empty());

    stack::push(1, &mut vm);
    assert_eq!(vm.stack(), &vec![18, 0, 0, 0].into());
}

#[test]
fn op_pull() {
    let mut vm = VM::new();
    vm.stack_mut().push_i32(42);
    assert_eq!(vm.stack(), &vec![42, 0, 0, 0].into());
    vm.registers_mut()[STACK_POINTER_REGISTER] = 4;

    stack::pop(3, &mut vm);

    assert_eq!(vm.registers()[3], 42);
    assert_eq!(vm.stack(), &vec![].into());
}

#[test]
fn op_pull_empty_stack() {
    let mut vm = VM::new();
    assert!(vm.stack().is_empty());
    vm.registers_mut()[2] = 5;

    stack::pop(2, &mut vm);
    assert_eq!(vm.registers()[2], 0);
}
