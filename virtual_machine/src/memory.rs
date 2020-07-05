use std::cell::UnsafeCell;

pub struct Memory {
    array: UnsafeCell<Vec<u8>>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            array: UnsafeCell::new(vec![]),
        }
    }

    fn grow(array: &mut Vec<u8>, index: usize) {
        while index >= array.len() {
            let old_len = array.len();
            let end = old_len * 2;

            for _ in old_len..end {
                array.push(0);
            }
        }
    }

    pub fn get_at(&self, index: usize) -> &[u8] {
        let array = unsafe { &mut *self.array.get() };

        Self::grow(array, index);

        &array[index..(index + 8)]
    }

    pub fn get_at_mut(&mut self, index: usize) -> &mut [u8] {
        let array = unsafe { &mut *self.array.get() };

        Self::grow(array, index);

        &mut array[index..(index + 8)]
    }
}
