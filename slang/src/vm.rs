use crate::instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    remainder: u32,

    pc: usize,
    program: Vec<u8>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            remainder: 0,

            pc: 0,
            program: Vec::new(),
        }
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
            return false;
        }

        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let value = self.next_16_bits() as u16;
                self.registers[register] = value as i32;
                true
            }
            Opcode::ADD => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register_1 + register_2;
                true
            }
            Opcode::SUB => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register_1 - register_2;
                true
            }
            Opcode::MUL => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register_1 * register_2;
                true
            }
            Opcode::DIV => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register_1 / register_2;
                self.remainder = (register_1 % register_2) as u32;
                true
            }
            Opcode::HLT => {
                println!("HLT received. Halting.");
                false
            }
            Opcode::IGL => {
                println!("Unknown opcode. Terminating");
                false
            }
        }
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
        assert_eq!(test_vm.pc, 1);
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
}
