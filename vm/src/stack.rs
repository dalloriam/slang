use std::mem;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[derive(Debug, PartialEq)]
pub struct Stack {
    data: Vec<u8>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { data: Vec::new() }
    }

    pub fn push_i32(&mut self, v: i32) {
        self.data.write_i32::<LittleEndian>(v).unwrap();
    }

    pub fn pop_i32(&mut self) -> i32 {
        if self.data.len() < (mem::size_of::<i32>() as usize) {
            return 0;
        }

        let stack_idx = self.data.len() - (mem::size_of::<i32>() as usize);
        let value = (&self.data[stack_idx..])
            .read_i32::<LittleEndian>()
            .unwrap(); // Impossible b/c length is at least size_of<i32>

        self.data.resize(stack_idx, 0);

        value
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl From<Vec<u8>> for Stack {
    fn from(v: Vec<u8>) -> Stack {
        Stack { data: v }
    }
}

impl Default for Stack {
    fn default() -> Stack {
        Stack::new()
    }
}
