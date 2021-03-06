#![deny(unused_imports)]

#[macro_use]
extern crate lazy_static;

mod flags;
pub mod instruction_set;
pub mod intrinsics;
pub mod memory;
pub mod registers;
pub mod resolution;
pub mod vm;

pub use vm::VirtualMachine;
