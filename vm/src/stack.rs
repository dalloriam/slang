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

    #[inline]
    pub fn push_i32(&mut self, v: i32) {
        self.data.write_i32::<LittleEndian>(v).unwrap();
    }

    #[inline]
    pub fn push_u8(&mut self, v: u8) {
        self.data.push(v);
    }

    #[inline]
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
    pub fn pop_u8(&mut self) -> u8 {
        if self.data.is_empty() {
            return 0;
        }
        self.data.pop().unwrap()
    }

    #[inline]
    pub fn memory(&self) -> &[u8] {
        &self.data
    }

    #[inline]
    pub fn memory_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    #[inline]
    pub fn safe_grow(&mut self, upper_bound: usize) {
        let abs_diff = (upper_bound as i32) - (self.data.len() as i32);
        if abs_diff > 0 {
            self.data.resize(upper_bound, 0);
        }
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
