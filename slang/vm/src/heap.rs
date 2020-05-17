pub struct MemoryBlock {
    data: Box<[u8]>,
    is_free: bool,
}

impl MemoryBlock {
    pub fn new(size: usize) -> MemoryBlock {
        let raw_bytes = vec![0; size];
        MemoryBlock {
            data: raw_bytes.into_boxed_slice(),
            is_free: false,
        }
    }

    pub fn data(&self) -> &Box<[u8]> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Box<[u8]> {
        &mut self.data
    }
}

/// Heap of the VM. Dynamically re-sizable through allocations.
pub struct Heap {
    data: Vec<MemoryBlock>,
}

impl Heap {
    pub fn new() -> Heap {
        Heap { data: Vec::new() }
    }

    fn align(n: usize) -> usize {
        // Align the provided size value with the platform size.
        // TODO: Implement.
        n
    }

    pub fn alloc(&mut self, mut size: usize) -> &mut MemoryBlock {
        size = Heap::align(size);

        let idx = match self
            .data
            .iter_mut()
            .position(|ref e| e.is_free && e.data.len() >= size)
        {
            Some(i) => i,
            None => {
                let new_block = MemoryBlock::new(size);
                self.data.push(new_block);
                self.data.len() - 1
            }
        };

        self.data.get_mut(idx).unwrap()
    }
}
