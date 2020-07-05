mod flags;
pub mod instruction_set;
mod memory;
pub mod registers;
pub mod vm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
