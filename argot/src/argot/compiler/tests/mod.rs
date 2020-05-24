use crate::Compiler;

#[test]
fn basic_add() {
    let src = "2 + 3";
    let asm = Compiler::new().compile_asm(src);
    let expected_asm = ".data
.text
ld $8 0x0002
ld $9 0x0003
add $8 $9 $0
move $0 $8";

    assert_eq!(asm, expected_asm);
}

#[test]
fn basic_mult() {
    let src = "2 * 3";
    let asm = Compiler::new().compile_asm(src);
    let expected_asm = ".data
.text
ld $8 0x0002
ld $9 0x0003
mul $8 $9 $0
move $0 $8";

    assert_eq!(asm, expected_asm);
}

#[test]
fn basic_sub() {
    let src = "3 - 2";
    let asm = Compiler::new().compile_asm(src);
    let expected_asm = ".data
.text
ld $8 0x0003
ld $9 0x0002
sub $8 $9 $0
move $0 $8";

    assert_eq!(asm, expected_asm);
}

#[test]
fn basic_div() {
    let src = "3 / 2";
    let asm = Compiler::new().compile_asm(src);
    let expected_asm = ".data
.text
ld $8 0x0003
ld $9 0x0002
div $8 $9 $0
move $0 $8";

    assert_eq!(asm, expected_asm);
}

#[test]
fn nested_add_mult() {
    let src = "3 * (2 + 2)";
    let asm = Compiler::new().compile_asm(src);
    let expected_asm = ".data
.text
ld $8 0x0003
ld $9 0x0002
ld $10 0x0002
add $9 $10 $0
move $0 $9
mul $8 $9 $0
move $0 $8";

    assert_eq!(asm, expected_asm);
}
