use crate::compiler::Compiler;

#[test]
fn compile_basic_program() {
    const TEST_EXPR: &str = "fn main() {int i = 1 + 3 * 2;}";

    let mut compiler = Compiler::new();
    let _asm = compiler.compile_asm(TEST_EXPR);
}
