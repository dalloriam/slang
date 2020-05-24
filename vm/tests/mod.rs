// VM functional tests.
#[cfg(test)]
mod tests {
    use byteorder::{LittleEndian, ReadBytesExt};

    use assembler::Assembler;
    use vm::{memutil, VM};

    fn execute_test(source: &str, max_epochs: usize) -> VM {
        let mut vm = VM::new();
        let asm = Assembler::new().assemble(source).unwrap();
        vm.load_bytecode(asm).unwrap();

        let mut current_epochs = 0;
        let mut keepalive = true;
        while keepalive {
            keepalive = vm.run_once();
            current_epochs += 1;
            if current_epochs >= max_epochs {
                panic!("Test took too long");
            }
        }

        vm
    }

    // TODO: Ensure all instructions are covered by the FTs as well.

    #[test]
    fn ft_alloc() {
        const SOURCE: &str = include_str!("./data/alloc.asm");
        let vm = execute_test(SOURCE, 10);
        let aligned_size = memutil::align(4);
        assert!(aligned_size >= 4);
        assert_eq!(vm.heap().memory().len(), aligned_size);
        assert_eq!(vm.registers()[10], 45);

        let v = vm.heap().memory().read_i32::<LittleEndian>().unwrap();
        assert_eq!(v, 45);
    }

    #[test]
    fn ft_loop() {
        const SOURCE: &str = include_str!("./data/loop.asm");
        let vm = execute_test(SOURCE, 40);
        assert_eq!(vm.registers()[0], 11);
    }

    #[test]
    fn ft_stack() {
        const SOURCE: &str = include_str!("./data/stack.asm");
        let vm = execute_test(SOURCE, 8);
        assert_eq!(vm.registers()[0], 20);
        assert_eq!(vm.registers()[1], 10);
    }

    #[test]
    fn ft_constr() {
        // Tests that constr loading parses & runs.
        const SOURCE: &str = include_str!("./data/constr.asm");
        let _vm = execute_test(SOURCE, 20);
    }
}
