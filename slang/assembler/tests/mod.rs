//! Functional tests for the assembler.
#[cfg(test)]
mod tests {
    use assembler::Assembler;

    #[test]
    pub fn add_test() {
        const SOURCE: &str = include_str!("./data/add.asm");
        const EXPECTED_ASM: &[u8] = include_bytes!("./data/add.bin");

        let actual_asm = Assembler::new().assemble(SOURCE);
        assert_eq!(actual_asm, EXPECTED_ASM);
    }

    #[test]
    pub fn symbol_test() {
        const SOURCE: &str = include_str!("./data/symbol.asm");
        const EXPECTED_ASM: &[u8] = include_bytes!("./data/symbol.bin");

        let actual_asm = Assembler::new().assemble(SOURCE);
        assert_eq!(actual_asm, EXPECTED_ASM);
    }
}
