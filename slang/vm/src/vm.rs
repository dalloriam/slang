use instructor::{Opcode, SysCall};

use snafu::{ResultExt, Snafu};

use crate::syscall::execute_syscall;

const REGISTER_COUNT: usize = 33;
const SYSCALL_REGISTER: usize = 32;

#[derive(Debug, Snafu)]
pub enum VMError {
    LoadingFailed { source: crate::loader::LoadError },
}

type Result<T> = std::result::Result<T, VMError>;

pub struct VM {
    // Registers 0-31 are regular registers. Reg 32 is the syscall register.
    registers: [i32; REGISTER_COUNT],
    ro_block: Vec<u8>,

    remainder: u32,
    equal_flag: bool,
    heap: Vec<u8>,

    pc: usize,
    program: Vec<u8>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; REGISTER_COUNT],
            ro_block: Vec::new(),
            remainder: 0,
            equal_flag: false,
            heap: Vec::new(),

            pc: 0,
            program: Vec::new(),
        }
    }

    pub fn program(&self) -> &[u8] {
        &self.program
    }

    pub fn erase_program(&mut self) {
        self.program.clear();
        self.pc = 0;
    }

    pub fn registers(&self) -> &[i32; 33] {
        // TODO: Make this transparent. Make it so it returns regular registers (0-31).
        // Return the other special registers from other methods.
        &self.registers
    }

    pub fn load_bytecode(&mut self, bytecode: Vec<u8>) -> Result<()> {
        let program = crate::loader::Program::new(bytecode).context(LoadingFailed)?;

        // TODO: Use program struct directly instead of unpacking.
        self.program = program.program_text;
        self.ro_block = program.ro_block;

        Ok(())
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;

        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;

        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;

        result
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            println!("End of program reached.");
            return false;
        }

        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let value = self.next_16_bits() as u16;
                self.registers[register] = value as i32;
            }
            Opcode::ADD => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register_1 + register_2;
            }
            Opcode::SUB => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register_1 - register_2;
            }
            Opcode::MUL => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register_1 * register_2;
            }
            Opcode::DIV => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register_1 / register_2;
                self.remainder = (register_1 % register_2) as u32;
            }
            Opcode::JMP => {
                // Short label jump.
                let target_idx = self.next_16_bits() as u16;

                // Eat last byte.
                self.next_8_bits();

                self.pc = target_idx as usize;
            }
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize];

                // Eat last two bytes.
                self.next_8_bits();
                self.next_8_bits();

                self.pc += value as usize;
            }
            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize];
                // Eat last two bytes.
                self.next_8_bits();
                self.next_8_bits();

                self.pc -= value as usize;
            }
            Opcode::RJMP => {
                // Long absolute jump.
                let value = self.registers[self.next_8_bits() as usize];
                // Eat last two bytes.
                self.next_8_bits();
                self.next_8_bits();
                self.pc = value as usize;
            }
            Opcode::EQ => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register_1 == register_2;
                self.next_8_bits();
            }
            Opcode::NEQ => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register_1 != register_2;
                self.next_8_bits();
            }
            Opcode::GT => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register_1 > register_2;
                self.next_8_bits();
            }
            Opcode::LT => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register_1 < register_2;
                self.next_8_bits();
            }
            Opcode::GTQ => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register_1 >= register_2;
                self.next_8_bits();
            }
            Opcode::LTQ => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register_1 <= register_2;
                self.next_8_bits();
            }
            Opcode::JEQ => {
                let target = self.next_16_bits() as u16;

                // Eat last byte.
                self.next_8_bits();

                if self.equal_flag {
                    self.pc = target as usize;
                }
            }
            Opcode::ALOC => {
                let amt_to_alloc = self.registers[self.next_8_bits() as usize];
                let new_heap_size = self.heap.len() as i32 + amt_to_alloc;
                self.heap.resize(new_heap_size as usize, 0);

                // Eat last two bytes.
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::INC => {
                self.registers[self.next_8_bits() as usize] += 1;

                // Eat last two bytes.
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::DEC => {
                self.registers[self.next_8_bits() as usize] -= 1;

                // Eat last two bytes.
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::SYSC => {
                // Execute a syscall.
                let call_id = SysCall::from(self.registers[SYSCALL_REGISTER]);
                let should_continue =
                    execute_syscall(call_id, &self.ro_block, &mut self.registers[0..32]);

                if !should_continue {
                    return false;
                }

                // Eat last three bytes.
                self.next_8_bits();
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::PUSH => unimplemented!(),
            Opcode::POP => unimplemented!(),
            Opcode::HLT => {
                println!("HLT received. Halting.");
                for _i in 0..3 {
                    // Eat remaining instruction bytes.
                    self.next_8_bits();
                }
                return false;
            }
            Opcode::IGL => {
                println!("Illegal opcode. Terminating");
                return false;
            }
        }
        true
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    pub fn run(&mut self) {
        let mut keepalive = true;
        while keepalive {
            keepalive = self.execute_instruction();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VM;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        for register in test_vm.registers.iter() {
            assert_eq!(*register, 0);
        }
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 244];

        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 8;
        test_vm.program = vec![2, 0, 1, 2];

        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 18);
    }

    #[test]
    fn test_opcode_sub() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 8;
        test_vm.program = vec![3, 0, 1, 2];

        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 2);
    }

    #[test]
    fn test_opcode_mul() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 8;
        test_vm.program = vec![4, 0, 1, 2];

        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 80);
    }

    #[test]
    fn test_opcode_div() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 8;
        test_vm.program = vec![5, 0, 1, 2];

        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 2);
        assert_eq!(test_vm.remainder, 4);
    }

    #[test]
    fn test_opcode_jmp() {
        let mut test_vm = VM::new();
        test_vm.program = vec![6, 0, 10, 0];
        test_vm.run_once();

        assert_eq!(test_vm.pc, 10);
    }

    #[test]
    fn test_opcode_jmpf() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 1, 0, 6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 6);
    }

    #[test]
    fn test_opcode_jmpb() {
        let mut test_vm = VM::new();
        test_vm.pc = 2;
        test_vm.registers[0] = 6;
        test_vm.program = vec![7, 0, 8, 0, 6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_opcode_eq() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];

        // Exec the first instruction -- should be equal.
        test_vm.run_once();
        assert!(test_vm.equal_flag);

        // Change one of the registers, next instruction should not be equal.
        test_vm.registers[0] = 9;
        test_vm.run_once();
        assert!(!test_vm.equal_flag);
    }

    #[test]
    fn test_opcode_neq() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![10, 0, 1, 0, 10, 0, 1, 0];

        // Exec the first instruction -- should not be equal.
        test_vm.run_once();
        assert!(!test_vm.equal_flag);

        // Change one of the registers, next instruction should be equal.
        test_vm.registers[0] = 9;
        test_vm.run_once();
        assert!(test_vm.equal_flag);
    }

    #[test]
    fn test_opcode_gt() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 11;
        test_vm.registers[1] = 10;
        test_vm.program = vec![11, 0, 1, 0, 11, 0, 1, 0];

        // Exec the first instruction -- should be equal.
        test_vm.run_once();
        assert!(test_vm.equal_flag);

        // Change one of the registers, next instruction not should be equal.
        test_vm.registers[0] = 9;
        test_vm.run_once();
        assert!(!test_vm.equal_flag);
    }

    #[test]
    fn test_opcode_lt() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 9;
        test_vm.registers[1] = 10;
        test_vm.program = vec![12, 0, 1, 0, 12, 0, 1, 0];

        // Exec the first instruction -- should be equal.
        test_vm.run_once();
        assert!(test_vm.equal_flag);

        // Change one of the registers, next instruction should not be equal.
        test_vm.registers[0] = 11;
        test_vm.run_once();
        assert!(!test_vm.equal_flag);
    }

    #[test]
    fn test_opcode_gtq() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 11;
        test_vm.registers[1] = 10;
        test_vm.program = vec![13, 0, 1, 0, 13, 0, 1, 0, 13, 0, 1, 0];

        test_vm.run_once();
        assert!(test_vm.equal_flag);

        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert!(test_vm.equal_flag);

        test_vm.registers[0] = 9;
        test_vm.run_once();
        assert!(!test_vm.equal_flag);
    }

    #[test]
    fn test_opcode_ltq() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 9;
        test_vm.registers[1] = 10;
        test_vm.program = vec![14, 0, 1, 0, 14, 0, 1, 0, 14, 0, 1, 0];

        test_vm.run_once();
        assert!(test_vm.equal_flag);

        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert!(test_vm.equal_flag);

        test_vm.registers[0] = 11;
        test_vm.run_once();
        assert!(!test_vm.equal_flag);
    }

    #[test]
    fn test_opcode_jeq() {
        let mut test_vm = VM::new();
        test_vm.equal_flag = true;
        test_vm.program = vec![15, 0, 7, 0, 16, 0, 0, 0, 16, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }

    #[test]
    fn test_opcode_aloc() {
        let mut test_vm = VM::new();

        test_vm.registers[0] = 1024;
        test_vm.program = vec![16, 0, 0, 0]; // Allocate 1kb.

        assert_eq!(test_vm.heap.len(), 0);
        test_vm.run_once();
        assert_eq!(test_vm.heap.len(), 1024);
    }

    #[test]
    fn test_opcode_inc() {
        let mut test_vm = VM::new();
        test_vm.program = vec![17, 0, 0, 0];

        assert_eq!(test_vm.registers[0], 0);
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 1);
    }

    #[test]
    fn test_opcode_dec() {
        let mut test_vm = VM::new();
        test_vm.program = vec![18, 0, 0, 0];

        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 9);
    }
}

impl Default for VM {
    fn default() -> VM {
        return VM {
            registers: [0; REGISTER_COUNT],
            ro_block: Vec::new(),
            remainder: 0,
            equal_flag: false,
            heap: Vec::new(),

            pc: 0,
            program: Vec::new(),
        };
    }
}
