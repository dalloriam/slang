#[derive(Debug, PartialEq)]
struct MemoryBlock {
    start_index: usize,
    size: usize,
    is_free: bool,
}

impl MemoryBlock {
    pub fn new(start_index: usize, size: usize) -> MemoryBlock {
        MemoryBlock {
            start_index,
            size,
            is_free: false,
        }
    }
}

/// Heap of the VM. Dynamically re-sizable through allocations.
pub struct Heap {
    index: Vec<MemoryBlock>,
    memory: Vec<u8>,
}

impl Heap {
    pub fn new() -> Heap {
        Heap {
            index: Vec::new(),
            memory: Vec::new(),
        }
    }

    #[inline]
    pub fn memory(&self) -> &[u8] {
        &self.memory
    }

    #[inline]
    pub fn memory_mut(&mut self) -> &mut [u8] {
        &mut self.memory
    }

    fn align(n: usize) -> usize {
        // Align the provided size value with the platform size.
        // TODO: Implement.
        n
    }

    pub fn alloc(&mut self, mut size: usize) -> usize {
        size = Heap::align(size);

        let idx = match self
            .index
            .iter_mut()
            .position(|ref e| e.is_free && e.size >= size)
        {
            Some(i) => {
                // We were able to find a block of free memory to re-allocate.
                // No memory resize necessary.
                self.index[i].is_free = false;
                i
            }
            None => {
                // No suitable free block was found.

                // We need to add a new memory block to the index to track the new memory that will be allocated.
                let new_block = MemoryBlock::new(self.memory.len(), size);
                self.index.push(new_block);
                let block_id = self.index.len() - 1;

                // Afterwards, we extend the available memory.
                self.memory.resize(self.memory.len() + size, 0);

                block_id
            }
        };

        let memory_address = self.index[idx].start_index;

        memory_address
    }

    pub fn free(&mut self, ptr: usize) {
        let idx = match self.index.iter().position(|e| e.start_index == ptr) {
            Some(i) => i,
            None => panic!("Invalid free"),
        };

        {
            let node = self.index.get_mut(idx).unwrap();
            if node.is_free {
                panic!("Double free");
            }
            node.is_free = true;
        }

        // Compaction.
        // We remove all data allocated to all free contiguous memory blocks, starting from the left.
        let mut amt_to_shrink = 0;
        for n in (0..self.index.len()).rev() {
            if self.index[n].is_free {
                amt_to_shrink += self.index[n].size;
            } else {
                break;
            }
        }
        self.memory.resize(self.memory.len() - amt_to_shrink, 0);
    }
}

#[cfg(test)]
mod tests {

    use super::{Heap, MemoryBlock};

    #[test]
    fn test_heap_alloc() {
        let mut heap = Heap::new();
        {
            // Allocate 4 bytes to the heap.
            let mut ptr = heap.alloc(4);
            for i in 0..4 {
                heap.memory[ptr] = i;
                ptr = ptr + 1;
            }
        }

        {
            let mut ptr = heap.alloc(2);
            for i in 0..2 {
                heap.memory[ptr] = 8 - i;
                ptr = ptr + 1;
            }
        }

        assert_eq!(heap.memory.len(), 6);
        assert_eq!(heap.memory, vec![0, 1, 2, 3, 8, 7]);
    }

    #[test]
    fn test_heap_simple_free() {
        let mut heap = Heap::new();
        heap.memory = vec![0, 1, 2, 3];
        heap.index = vec![MemoryBlock::new(0, 4)];

        // When a single block of memory is allocated,
        // it can be resized immediately upon free().
        heap.free(0);

        assert_eq!(heap.memory.len(), 0);
    }

    #[test]
    fn test_heap_contiguous_free() {
        let mut heap = Heap::new();
        heap.memory = vec![0, 1, 2, 3, 8, 7];
        heap.index = vec![MemoryBlock::new(0, 4), MemoryBlock::new(4, 2)];

        // Since the [4-6] block is still allocated, freeing the [0-4] block won't shrink the process memory.
        heap.free(0);
        assert_eq!(heap.memory.len(), 6);
        assert!(heap.index[0].is_free);

        // However, if we free the [4-6] block too, the whole memory should be freed.
        heap.free(4);
        assert_eq!(heap.memory.len(), 0);
    }

    #[test]
    #[should_panic(expected = "Double free")]
    fn test_heap_double_free() {
        let mut heap = Heap::new();
        heap.memory = vec![0, 1];
        heap.index = vec![MemoryBlock::new(0, 2)];

        heap.free(0);
        heap.free(0);
    }

    #[test]
    #[should_panic(expected = "Invalid free")]
    fn test_heap_invalid_free() {
        let mut heap = Heap::new();
        heap.memory = vec![0, 1];
        heap.index = vec![MemoryBlock::new(0, 2)];

        heap.free(18);
    }
}
