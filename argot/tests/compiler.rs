macro_rules! compiler_fts {
    ($($name:ident,)*) => {
        $(
            #[test]
            fn $name() {
                const SOURCE: &str = include_str!(concat!("data/", stringify!($name), ".gt"));
                const EXPECTED_ASM: &str = include_str!(concat!("data/", stringify!($name), ".asm"));
                let actual_asm = argot::compile_asm(SOURCE).unwrap();
                assert_eq!(EXPECTED_ASM.trim(), actual_asm.trim());
            }
        )*
    }
}

compiler_fts! {
    add,
    fn_call_no_ret,
    mult,
    nested_math,
    var_decl,
    var_usage,
    neg_int,
    var_ref,
    simple_if,
    if_else,
    stack_fallback,
}
