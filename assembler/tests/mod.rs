//! Functional tests for the assembler.
#[cfg(test)]
mod tests {
    use assembler::Assembler;

    #[test]
    pub fn ft_add() {
        const SOURCE: &str = include_str!("./data/add.asm");
        const EXPECTED_ASM: &[u8] = include_bytes!("./data/add.bin");

        let actual_asm = Assembler::new().assemble(SOURCE).unwrap();
        assert_eq!(actual_asm, EXPECTED_ASM);
    }

    #[test]
    pub fn ft_symbol() {
        const SOURCE: &str = include_str!("./data/symbol.asm");
        const EXPECTED_ASM: &[u8] = include_bytes!("./data/symbol.bin");

        let actual_asm = Assembler::new().assemble(SOURCE).unwrap();
        assert_eq!(actual_asm, EXPECTED_ASM);
    }

    #[test]
    pub fn ft_word_directive() {
        const SOURCE: &str = include_str!("./data/word_directive.asm");
        const EXPECTED_ASM: &[u8] = include_bytes!("./data/word_directive.bin");

        let actual_asm = Assembler::new().assemble(SOURCE).unwrap();
        assert_eq!(actual_asm, EXPECTED_ASM);
    }
}
