use crate::Compiler;

#[test]
fn basic_add() {
    let src = "2 + 3";
    let asm = Compiler::new().compile_asm(src);
    let expected_asm = ".data
.text
ld $31 0x0002
ld $30 0x0003
add $31 $30 $29";

    assert_eq!(asm, expected_asm);
}

#[test]
fn basic_mult() {
    let src = "2 * 3";
    let asm = Compiler::new().compile_asm(src);
    let expected_asm = ".data
.text
ld $31 0x0002
ld $30 0x0003
mul $31 $30 $29";

    assert_eq!(asm, expected_asm);
}

#[test]
fn basic_sub() {
    let src = "3 - 2";
    let asm = Compiler::new().compile_asm(src);
    let expected_asm = ".data
.text
ld $31 0x0003
ld $30 0x0002
sub $31 $30 $29";

    assert_eq!(asm, expected_asm);
}

#[test]
fn basic_div() {
    let src = "3 / 2";
    let asm = Compiler::new().compile_asm(src);
    let expected_asm = ".data
.text
ld $31 0x0003
ld $30 0x0002
div $31 $30 $29";

    assert_eq!(asm, expected_asm);
}

#[test]
fn nested_add_mult() {
    let src = "3 * (2 + 2)";
    let asm = Compiler::new().compile_asm(src);
    let expected_asm = ".data
.text
ld $31 0x0003
ld $30 0x0002
ld $29 0x0002
add $30 $29 $28
mul $31 $28 $30";

    assert_eq!(asm, expected_asm);
}
