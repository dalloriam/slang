use argot::Compiler;

macro_rules! compiler_fts {
    ($($name:ident,)*) => {
        $(
            #[test]
            fn $name() {
                const SOURCE: &str = include_str!(concat!("data/", stringify!($name), ".gt"));
                const EXPECTED_ASM: &str = include_str!(concat!("data/", stringify!($name), ".asm"));
                let actual_asm = Compiler::new().compile_asm(SOURCE).unwrap();
                assert_eq!(EXPECTED_ASM.trim(), actual_asm.trim());
            }
        )*
    }
}

compiler_fts! {
    empty_program,
    add,
    mult,
    nested_math,
    var_decl,
    var_usage,
    neg_int,
}
