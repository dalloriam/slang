use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

use instructor::{Address, MemorySection, Opcode, SysCall};

use snafu::{ResultExt, Snafu};

use crate::constants::{REGISTER_COUNT, SYSCALL_REGISTER};
use crate::heap::Heap;
use crate::op;
use crate::stack::Stack;
use crate::syscall::execute_syscall;

#[derive(Debug, Snafu)]
pub enum VMError {
    LoadingError { source: crate::loader::LoadError },
}

type Result<T> = std::result::Result<T, VMError>;

pub struct VM {
    // Registers 0-31 are regular registers. Reg 32 is the syscall register.
    registers: [i32; REGISTER_COUNT],
    ro_block: Vec<u8>,

    remainder: u32,
    pub equal_flag: bool,

    stack: Stack,
    heap: Heap,

    pub pc: usize,
    program: Vec<u8>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; REGISTER_COUNT],
            ro_block: Vec::new(),
            remainder: 0,
            equal_flag: false,
            stack: Stack::new(),
            heap: Heap::new(),

            pc: 0,
            program: Vec::new(),
        }
    }

    pub fn with_ro_block(ro: Vec<u8>) -> VM {
        VM {
            ro_block: ro,
            ..Default::default()
        }
    }

    pub fn program(&self) -> &[u8] {
        &self.program
    }

    pub fn erase_program(&mut self) {
        self.program.clear();
        self.pc = 0;
    }

    #[inline]
    pub fn registers(&self) -> &[i32; REGISTER_COUNT] {
        // TODO: Make this transparent. Make it so it returns regular registers (0-31).
        // Return the other special registers from other methods.
        &self.registers
    }

    #[inline]
    pub fn registers_mut(&mut self) -> &mut [i32; REGISTER_COUNT] {
        &mut self.registers
    }

    #[inline]
    pub fn ro_block(&self) -> &[u8] {
        &self.ro_block
    }

    #[inline]
    pub fn stack(&self) -> &Stack {
        &self.stack
    }

    #[inline]
    pub fn stack_mut(&mut self) -> &mut Stack {
        &mut self.stack
    }

    #[inline]
    pub fn heap(&self) -> &Heap {
        &self.heap
    }

    #[inline]
    pub fn heap_mut(&mut self) -> &mut Heap {
        &mut self.heap
    }

    #[inline]
    pub fn remainder(&self) -> u32 {
        self.remainder
    }

    #[inline]
    pub fn set_remainder(&mut self, v: u32) {
        self.remainder = v;
    }

    pub fn load_bytecode(&mut self, bytecode: Vec<u8>) -> Result<()> {
        let program = crate::loader::Program::new(bytecode).context(LoadingSnafu)?;

        // TODO: Use program struct directly instead of unpacking.
        self.program = program.program_text;
        self.ro_block = program.ro_block;

        Ok(())
    }

    #[inline]
    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;

        opcode
    }

    #[inline]
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;

        result
    }

    #[inline]
    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;

        result
    }

    #[inline]
    fn next_i32(&mut self) -> i32 {
        let mut rdr = Cursor::new(&self.program[self.pc..self.pc + 4]);
        let v = rdr.read_i32::<LittleEndian>().unwrap(); // TODO: Handle
        self.pc += 4;
        v
    }

    #[inline]
    fn next_address(&mut self) -> Address {
        let register = self.next_8_bits();
        let offset = self.next_i32();
        let section = MemorySection::from(self.next_8_bits());

        Address {
            register,
            offset,
            section,
        }
    }

    pub fn run_once(&mut self) -> bool {
        self.execute_instruction()
    }

    pub fn run(&mut self) {
        let start = std::time::Instant::now();
        let mut keepalive = true;
        while keepalive {
            keepalive = self.execute_instruction();
        }

        self.post_run_validations();

        let dur = std::time::Instant::now().duration_since(start);
        println!("Done in {}us", dur.as_micros());
    }

    fn post_run_validations(&self) {
        log::debug!("running debug validations");

        log::debug!("validating that stack is empty");
        debug_assert_eq!(self.stack.len(), 0);
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            println!("End of program reached.");
            return false;
        }

        match self.decode_opcode() {
            Opcode::LOAD => op::reg::load(self.next_8_bits(), self.next_16_bits(), self),
            Opcode::ADD => op::math::add(
                self.next_8_bits(),
                self.next_8_bits(),
                self.next_8_bits(),
                self,
            ),
            Opcode::SUB => op::math::sub(
                self.next_8_bits(),
                self.next_8_bits(),
                self.next_8_bits(),
                self,
            ),
            Opcode::MUL => op::math::mul(
                self.next_8_bits(),
                self.next_8_bits(),
                self.next_8_bits(),
                self,
            ),
            Opcode::DIV => op::math::div(
                self.next_8_bits(),
                self.next_8_bits(),
                self.next_8_bits(),
                self,
            ),
            Opcode::JMP => op::branch::jmp(self.next_16_bits(), self),
            Opcode::JMPF => op::branch::jmpf(self.next_16_bits(), self),
            Opcode::JMPB => op::branch::jmpb(self.next_16_bits(), self),
            Opcode::RJMP => op::branch::rjmp(self.next_8_bits(), self),
            Opcode::EQ => op::math::eq(self.next_8_bits(), self.next_8_bits(), self),
            Opcode::NEQ => op::math::neq(self.next_8_bits(), self.next_8_bits(), self),
            Opcode::GT => op::math::gt(self.next_8_bits(), self.next_8_bits(), self),
            Opcode::LT => op::math::lt(self.next_8_bits(), self.next_8_bits(), self),
            Opcode::GTQ => op::math::gtq(self.next_8_bits(), self.next_8_bits(), self),
            Opcode::LTQ => op::math::ltq(self.next_8_bits(), self.next_8_bits(), self),
            Opcode::JEQ => op::branch::jeq(self.next_16_bits(), self),
            Opcode::INC => op::reg::inc(self.next_8_bits(), self),
            Opcode::DEC => op::reg::dec(self.next_8_bits(), self),
            Opcode::SYSC => {
                // Execute a syscall.
                log::trace!("syscall {:#06x}", self.registers[SYSCALL_REGISTER]);
                let call_id = SysCall::from(self.registers[SYSCALL_REGISTER]);
                let should_continue = execute_syscall(call_id, self);

                if !should_continue {
                    return false;
                }
            }
            Opcode::PUSHW => op::stack::pushw(self.next_8_bits(), self),
            Opcode::POPW => op::stack::popw(self.next_8_bits(), self),
            Opcode::MOV => op::reg::mov(self.next_8_bits(), self.next_8_bits(), self),
            Opcode::LCW => op::ro::lcw(self.next_8_bits(), self.next_16_bits(), self),
            Opcode::SW => op::memory::sw(self.next_8_bits(), &self.next_address(), self),
            Opcode::LW => op::memory::lw(self.next_8_bits(), &self.next_address(), self),
            Opcode::SB => op::memory::sb(self.next_8_bits(), &self.next_address(), self),
            Opcode::LB => op::memory::lb(self.next_8_bits(), &self.next_address(), self),
            Opcode::CALL => op::branch::call(self.next_16_bits(), self),
            Opcode::RET => op::branch::ret(self),
            Opcode::NEG => op::math::neg(self.next_8_bits(), self),
            Opcode::PUSHB => op::stack::pushb(self.next_8_bits(), self),
            Opcode::POPB => op::stack::popb(self.next_8_bits(), self),
            Opcode::JEZ => op::branch::jez(self.next_8_bits(), self.next_16_bits(), self),
            Opcode::NOT => op::bitwise::not(self.next_8_bits(), self),
            Opcode::SHIFTL => op::bitwise::shiftl(self.next_8_bits(), self.next_8_bits(), self),
            Opcode::SHIFTR => op::bitwise::shiftr(self.next_8_bits(), self.next_8_bits(), self),
            Opcode::AND => op::bitwise::and(self.next_8_bits(), self.next_8_bits(), self),
            Opcode::OR => op::bitwise::or(self.next_8_bits(), self.next_8_bits(), self),
            Opcode::IGL => {
                println!("Illegal opcode. Terminating");
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use byteorder::{LittleEndian, WriteBytesExt};

    use super::VM;
    use instructor::STACK_POINTER_REGISTER;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        for register in test_vm.registers.iter() {
            assert_eq!(*register, 0);
        }
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 0, 0, 0];
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
        test_vm.program = vec![7, 0, 2, 6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 5);
    }

    #[test]
    fn test_opcode_jmpb() {
        let mut test_vm = VM::new();
        test_vm.pc = 2;
        test_vm.program = vec![7, 0, 8, 0, 5];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_opcode_eq() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![9, 0, 1, 9, 0, 1];

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
        test_vm.program = vec![10, 0, 1, 10, 0, 1];

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
        test_vm.program = vec![11, 0, 1, 11, 0, 1];

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
        test_vm.program = vec![12, 0, 1, 12, 0, 1];

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
    fn test_opcode_inc() {
        let mut test_vm = VM::new();
        test_vm.program = vec![16, 0, 0, 0];

        assert_eq!(test_vm.registers[0], 0);
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 1);
    }

    #[test]
    fn test_opcode_dec() {
        let mut test_vm = VM::new();
        test_vm.program = vec![17, 0, 0, 0];

        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 9);
    }

    #[test]
    fn test_opcode_push() {
        let mut test_vm = VM::new();
        assert!(test_vm.stack.is_empty());
        test_vm.registers[0] = 12;
        test_vm.program = vec![20, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.stack, vec![12, 0, 0, 0].into());
    }

    #[test]
    fn test_opcode_pop() {
        let mut test_vm = VM::new();
        test_vm.stack = vec![18, 0, 0, 0, 32, 0, 0, 0].into();
        test_vm.registers[STACK_POINTER_REGISTER] = 8;

        test_vm.program = vec![21, 0, 0, 0];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 32);
        assert_eq!(test_vm.stack, vec![18, 0, 0, 0].into());
    }

    #[test]
    fn test_opcode_mov() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 18;

        assert_eq!(test_vm.registers[0], 0);
        test_vm.program = vec![22, 1, 0, 0];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 18);
    }

    #[test]
    fn test_opcode_lcw() {
        let mut test_vm = VM::new();
        test_vm.ro_block = vec![0, 0, 0, 0, 10, 0, 0, 0];

        assert_eq!(test_vm.registers[0], 0);
        test_vm.program = vec![23, 0, 0, 4, 1];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 10);
    }

    #[test]
    fn test_opcode_sw() {
        let mut test_vm = VM::new();

        test_vm.heap_mut().alloc(8);

        assert_eq!(&test_vm.heap().memory(), &vec![0; 8].as_slice());

        test_vm.registers[0] = 512;
        test_vm.registers[1] = 4;
        test_vm.program = vec![24, 0, 1, 0, 0, 0, 0, 1];
        test_vm.run_once();

        assert_eq!(
            &test_vm.heap().memory(),
            &vec![0, 0, 0, 0, 0, 2, 0, 0].as_slice()
        );
    }

    #[test]
    fn test_opcode_lw() {
        let mut test_vm = VM::new();

        // Allocate 8 bytes in memory.
        test_vm.heap_mut().alloc(8);

        // Write an int to the second 4-byte memory block.
        (&mut test_vm.heap_mut().memory_mut()[4..])
            .write_i32::<LittleEndian>(42)
            .unwrap();

        // Try to fetch it back with an lw instruction.
        test_vm.registers[1] = 4; // Pointer to the memory location containing our number.
        test_vm.program = vec![25, 0, 1, 0, 0, 0, 0, 1];

        assert_eq!(test_vm.registers[0], 0);
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 42);
    }

    #[test]
    fn test_opcode_sb() {
        let mut test_vm = VM::new();

        test_vm.heap_mut().alloc(4);

        assert_eq!(test_vm.heap().memory()[3], 0);

        test_vm.registers[0] = 42;
        test_vm.registers[1] = 3;
        test_vm.program = vec![26, 0, 1, 0, 0, 0, 0, 1];
        test_vm.run_once();

        assert_eq!(test_vm.heap().memory()[3], 42)
    }

    #[test]
    fn test_opcode_lb() {
        let mut test_vm = VM::new();

        test_vm.heap_mut().alloc(4);
        test_vm.heap_mut().memory_mut()[2] = 42;

        test_vm.registers[1] = 2;
        test_vm.program = vec![27, 0, 1, 0, 0, 0, 0, 1];

        assert_eq!(test_vm.registers[0], 0);
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 42);
    }

    #[test]
    fn test_opcode_neg() {
        let mut test_vm = VM::new();

        test_vm.registers_mut()[15] = 18;
        test_vm.program = vec![30, 15];
        test_vm.run_once();
        assert_eq!(test_vm.registers()[15], -18);
    }

    #[test]
    fn test_opcode_pushb() {
        let mut test_vm = VM::new();

        test_vm.registers_mut()[15] = 14;
        test_vm.program = vec![31, 15];
        test_vm.run_once();
        assert_eq!(test_vm.stack().memory(), vec![14].as_slice());
    }

    #[test]
    fn test_opcode_popb() {
        let mut test_vm = VM::new();

        test_vm.stack_mut().push_u8(42);
        test_vm.registers_mut()[STACK_POINTER_REGISTER] = 1;

        test_vm.program = vec![32, 10];
        test_vm.run_once();

        assert_eq!(test_vm.registers()[10], 42);
        assert_eq!(test_vm.stack().len(), 0);
    }

    #[test]
    fn test_opcode_not() {
        let mut test_vm = VM::new();

        test_vm.program = vec![33, 15];
        test_vm.registers_mut()[15] = 0x0000002a;
        test_vm.run_once();
        assert_eq!(test_vm.registers()[15], -43);
    }

    #[test]
    fn test_opcode_shiftl() {
        let mut test_vm = VM::new();

        test_vm.program = vec![34, 15, 10];

        test_vm.registers_mut()[15] = 21;
        test_vm.registers_mut()[10] = 1;

        test_vm.run_once();

        assert_eq!(test_vm.registers_mut()[10], 1);
        assert_eq!(test_vm.registers_mut()[15], 42);
    }

    #[test]
    fn test_opcode_shiftr() {
        let mut test_vm = VM::new();

        test_vm.program = vec![35, 15, 10];

        test_vm.registers_mut()[15] = 84;
        test_vm.registers_mut()[10] = 1;

        test_vm.run_once();

        assert_eq!(test_vm.registers_mut()[10], 1);
        assert_eq!(test_vm.registers_mut()[15], 42);
    }

    #[test]
    fn test_opcode_and() {
        let mut test_vm = VM::new();

        test_vm.program = vec![36, 15, 10];

        test_vm.registers_mut()[15] = 59;
        test_vm.registers_mut()[10] = 46;

        test_vm.run_once();

        assert_eq!(test_vm.registers_mut()[10], 46);
        assert_eq!(test_vm.registers_mut()[15], 42);
    }

    #[test]
    fn test_opcode_or() {
        let mut test_vm = VM::new();

        test_vm.program = vec![37, 15, 10];

        test_vm.registers_mut()[15] = 8;
        test_vm.registers_mut()[10] = 34;

        test_vm.run_once();

        assert_eq!(test_vm.registers_mut()[10], 34);
        assert_eq!(test_vm.registers_mut()[15], 42);
    }
}

impl Default for VM {
    fn default() -> VM {
        VM {
            registers: [0; REGISTER_COUNT],
            ro_block: Vec::new(),
            remainder: 0,
            equal_flag: false,
            stack: Stack::new(),
            heap: Heap::new(),

            pc: 0,
            program: Vec::new(),
        }
    }
}
