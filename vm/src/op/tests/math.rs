use crate::op::math;
use crate::VM;

#[test]
fn op_add_3reg() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 3;
    vm.registers_mut()[1] = 5;
    vm.registers_mut()[2] = 0;

    math::add(0, 1, 2, &mut vm);
    assert_eq!(&vm.registers()[0..3], vec![3, 5, 8].as_slice());
}

#[test]
fn op_add_2reg() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 3;
    vm.registers_mut()[1] = 5;
    math::add(0, 1, 0, &mut vm);
    assert_eq!(&vm.registers()[0..2], vec![8, 5].as_slice());
}

#[test]
fn op_sub() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 8;
    vm.registers_mut()[1] = 3;
    vm.registers_mut()[2] = 0;
    math::sub(0, 1, 2, &mut vm);
    assert_eq!(&vm.registers()[0..3], vec![8, 3, 5].as_slice());
}

#[test]
fn op_sub_neg() {
    // Even though we have no way as of now to parse negative numbers,
    // they must work in the registers themselves.
    let mut vm = VM::new();
    vm.registers_mut()[0] = 5;
    vm.registers_mut()[1] = 8;
    vm.registers_mut()[2] = 0;
    math::sub(0, 1, 2, &mut vm);
    assert_eq!(&vm.registers()[0..3], vec![5, 8, -3].as_slice());
}

#[test]
fn op_mul() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 5;
    vm.registers_mut()[1] = 4;
    vm.registers_mut()[2] = 0;
    math::mul(0, 1, 2, &mut vm);

    assert_eq!(&vm.registers()[0..3], vec![5, 4, 20].as_slice());
}

#[test]
fn op_div_nonzero() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 8;
    vm.registers_mut()[1] = 3;
    vm.registers_mut()[2] = 0;
    math::div(0, 1, 2, &mut vm);

    assert_eq!(&vm.registers()[0..3], vec![8, 3, 2].as_slice());
    assert_eq!(vm.remainder(), 2);
}

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn op_div_zero() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 1;
    vm.registers_mut()[1] = 0;
    vm.registers_mut()[2] = 0;
    math::div(0, 1, 2, &mut vm);
}

#[test]
fn op_eq_true() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 10;
    vm.registers_mut()[1] = 10;
    math::eq(0, 1, &mut vm);

    assert_eq!(&vm.registers()[0..2], vec![10, 10].as_slice());
    assert!(vm.equal_flag);
}

#[test]
fn op_eq_false() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 10;
    vm.registers_mut()[1] = 11;
    math::eq(0, 1, &mut vm);

    assert_eq!(&vm.registers()[0..2], vec![10, 11].as_slice());
    assert!(!vm.equal_flag);
}

#[test]
fn op_neq_false() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 10;
    vm.registers_mut()[1] = 10;
    math::neq(0, 1, &mut vm);

    assert_eq!(&vm.registers()[0..2], vec![10, 10].as_slice());
    assert!(!vm.equal_flag);
}

#[test]
fn op_neq_true() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 10;
    vm.registers_mut()[1] = 11;
    math::neq(0, 1, &mut vm);

    assert_eq!(&vm.registers()[0..2], vec![10, 11].as_slice());
    assert!(vm.equal_flag);
}

#[test]
fn op_gt_greater() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 11;
    vm.registers_mut()[1] = 10;
    math::gt(0, 1, &mut vm);

    assert_eq!(&vm.registers()[0..2], vec![11, 10].as_slice());
    assert!(vm.equal_flag);
}

#[test]
fn op_gt_equal() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 11;
    vm.registers_mut()[1] = 11;
    math::gt(0, 1, &mut vm);

    assert_eq!(&vm.registers()[0..2], vec![11, 11].as_slice());
    assert!(!vm.equal_flag);
}

#[test]
fn op_gt_lower() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 9;
    vm.registers_mut()[1] = 11;
    math::gt(0, 1, &mut vm);

    assert_eq!(&vm.registers()[0..2], vec![9, 11].as_slice());
    assert!(!vm.equal_flag);
}

#[test]
fn op_gtq_greater() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 11;
    vm.registers_mut()[1] = 10;
    math::gtq(0, 1, &mut vm);

    assert_eq!(&vm.registers()[0..2], vec![11, 10].as_slice());
    assert!(vm.equal_flag);
}

#[test]
fn op_gtq_equal() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 11;
    vm.registers_mut()[1] = 11;
    math::gtq(0, 1, &mut vm);

    assert_eq!(&vm.registers()[0..2], vec![11, 11].as_slice());
    assert!(vm.equal_flag);
}

#[test]
fn op_gtq_lower() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 9;
    vm.registers_mut()[1] = 11;
    math::gtq(0, 1, &mut vm);

    assert_eq!(&vm.registers()[0..2], vec![9, 11].as_slice());
    assert!(!vm.equal_flag);
}

#[test]
fn op_lt_greater() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 11;
    vm.registers_mut()[1] = 10;
    math::lt(0, 1, &mut vm);

    assert_eq!(&vm.registers()[0..2], vec![11, 10].as_slice());
    assert!(!vm.equal_flag);
}

#[test]
fn op_lt_equal() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 11;
    vm.registers_mut()[1] = 11;
    math::lt(0, 1, &mut vm);

    assert_eq!(&vm.registers()[0..2], vec![11, 11].as_slice());
    assert!(!vm.equal_flag);
}

#[test]
fn op_lt_lower() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 9;
    vm.registers_mut()[1] = 11;
    math::lt(0, 1, &mut vm);

    assert_eq!(&vm.registers()[0..2], vec![9, 11].as_slice());
    assert!(vm.equal_flag);
}

#[test]
fn op_ltq_greater() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 11;
    vm.registers_mut()[1] = 10;
    math::ltq(0, 1, &mut vm);

    assert_eq!(&vm.registers()[0..2], vec![11, 10].as_slice());
    assert!(!vm.equal_flag);
}

#[test]
fn op_ltq_equal() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 11;
    vm.registers_mut()[1] = 11;
    math::ltq(0, 1, &mut vm);

    assert_eq!(&vm.registers()[0..2], vec![11, 11].as_slice());
    assert!(vm.equal_flag);
}

#[test]
fn op_ltq_lower() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 9;
    vm.registers_mut()[1] = 11;
    math::ltq(0, 1, &mut vm);

    assert_eq!(&vm.registers()[0..2], vec![9, 11].as_slice());
    assert!(vm.equal_flag);
}

#[test]
fn op_neg_positive() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = 10;
    math::neg(0, &mut vm);
    assert_eq!(vm.registers()[0], -10);
}

#[test]
fn op_neg_negative() {
    let mut vm = VM::new();
    vm.registers_mut()[0] = -10;
    math::neg(0, &mut vm);
    assert_eq!(vm.registers()[0], 10);
}

#[test]
fn op_neg_zero() {
    let mut vm = VM::new();
    assert_eq!(vm.registers()[0], 0);
    math::neg(0, &mut vm);
    assert_eq!(vm.registers()[0], 0);
}
