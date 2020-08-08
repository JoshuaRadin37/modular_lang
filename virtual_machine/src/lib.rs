#![deny(unused_imports)]

mod flags;
pub mod instruction_set;
pub mod memory;
pub mod registers;
pub mod resolution;
pub mod vm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
