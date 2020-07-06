use std::cell::UnsafeCell;
use std::iter::Iterator;
use std::ops::Range;

pub const MAX_MEM: usize = std::usize::MAX;
const MID: usize = MAX_MEM / 2;

pub struct Memory {
    high: UnsafeCell<Vec<u8>>,
    low: UnsafeCell<Vec<u8>>,
}

#[derive(Clone)]
enum MemLocation {
    High,
    Low,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            high: UnsafeCell::new(vec![]),
            low: UnsafeCell::new(vec![]),
        }
    }

    fn grow(array: &mut Vec<u8>, index: usize) {
        while index + 8 >= array.len() {
            let old_len = array.len();
            let end = old_len * 2;

            for _ in old_len..end {
                array.push(0);
            }
        }
    }

    fn get_vector_for_index(&self, index: usize) -> (&mut Vec<u8>, MemLocation) {
        if index <= MID {
            (unsafe { &mut *self.low.get() }, MemLocation::Low)
        } else {
            (unsafe { &mut *self.high.get() }, MemLocation::High)
        }
    }

    fn get_vector(&self, location: MemLocation) -> &mut Vec<u8> {
        match location {
            MemLocation::High => unsafe { &mut *self.high.get() },
            MemLocation::Low => unsafe { &mut *self.low.get() },
        }
    }

    fn adjusted_index(in_index: usize, location: MemLocation) -> usize {
        match location {
            MemLocation::High => MAX_MEM - in_index,
            MemLocation::Low => in_index,
        }
    }

    fn adjusted_range(in_index: usize, location: MemLocation) -> Range<usize> {
        match location {
            MemLocation::High => {
                let base = MAX_MEM - in_index;
                base..(base + 8)
            }
            MemLocation::Low => in_index..(in_index + 8),
        }
    }

    fn adjusted_range_sized(in_index: usize, length: usize, location: MemLocation) -> Range<usize> {
        match location {
            MemLocation::High => {
                let base = MAX_MEM - in_index;
                base..(base + length)
            }
            MemLocation::Low => in_index..(in_index + length),
        }
    }

    fn get_bytes_mut(&self, range: Range<usize>, location: MemLocation) -> Vec<&mut u8> {
        let mut output = vec![];
        match location {
            MemLocation::High => {
                let array = self.get_vector(location);
                let bytes = &mut array[range];
                bytes.reverse();
                for byte in bytes.iter_mut() {
                    output.push(byte);
                }
            }
            MemLocation::Low => {
                let array = self.get_vector(location);
                let bytes = &mut array[range];
                for byte in bytes.iter_mut() {
                    output.push(byte);
                }
            }
        }
        output
    }

    fn get_bytes(&self, range: Range<usize>, location: MemLocation) -> [u8; 8] {
        let mut output: [u8; 8] = [0; 8];
        match location {
            MemLocation::High => {
                let array = self.get_vector(location);
                let bytes = &mut array[range];
                bytes.reverse();
                for (index, byte) in bytes.iter_mut().enumerate() {
                    output[index] = *byte;
                }
            }
            MemLocation::Low => {
                let array = self.get_vector(location);
                let bytes = &mut array[range];
                for (index, byte) in bytes.iter_mut().enumerate() {
                    output[index] = *byte;
                }
            }
        }
        output
    }

    pub fn get_at(&self, index: usize) -> [u8; 8] {
        let (array, loc) = self.get_vector_for_index(index);
        let index = Self::adjusted_index(index, loc.clone());
        Self::grow(array, index);
        let range = Self::adjusted_range(index, loc.clone());
        self.get_bytes(range, loc)
    }

    pub fn get_at_mut(&mut self, index: usize) -> Vec<&mut u8> {
        let (array, loc) = self.get_vector_for_index(index);
        let index = Self::adjusted_index(index, loc.clone());
        Self::grow(array, index);
        let range = Self::adjusted_range(index, loc.clone());
        self.get_bytes_mut(range, loc)
    }

    pub fn get_at_of_size(&self, index: usize, length: usize) -> [u8; 8] {
        let (array, loc) = self.get_vector_for_index(index);
        let index = Self::adjusted_index(index, loc.clone());
        Self::grow(array, index);
        let range = Self::adjusted_range_sized(index, length, loc.clone());
        self.get_bytes(range, loc)
    }

    pub fn get_at_of_size_mut(&mut self, index: usize, length: usize) -> Vec<&mut u8> {
        let (array, loc) = self.get_vector_for_index(index);
        let index = Self::adjusted_index(index, loc.clone());
        Self::grow(array, index);
        let range = Self::adjusted_range_sized(index, length, loc.clone());
        self.get_bytes_mut(range, loc)
    }
}
