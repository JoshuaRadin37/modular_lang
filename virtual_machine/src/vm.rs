use crate::instruction_set::{Instruction, Immediate};
use std::cmp::Ordering;
use crate::vm::Fault::InvalidReturn;

pub struct VirtualMachine {
    instructions: Vec<Instruction>,
    program_counter: usize,
    heaps: Vec<Vec<u8>>,
    stack: Vec<Immediate>,
    catch_stack: Vec<usize>,
    ret_stack: Vec<usize>,
    compare_result: Option<bool>
}

pub enum Fault {
    InvalidReturn
}

impl VirtualMachine {

    pub fn new() -> Self {
        Self {
            instructions: vec![],
            program_counter: !0,
            heaps: vec![],
            stack: vec![],
            catch_stack: vec![],
            ret_stack: vec![],
            compare_result: None
        }
    }

    fn run_instruction(&mut self, instruction: &Instruction) -> Result<(), Fault> {
        let mut next_program_counter = self.program_counter + 1;
        match instruction {
            Instruction::Push(immediate) => {
                self.stack.push(*immediate);
            },
            Instruction::Pop => {
                self.stack.pop();
            },
            Instruction::Ret => {
                next_program_counter = self.ret_stack.pop().ok_or(InvalidReturn)? + 1;
            },
            Instruction::Jump(counter) => {
                next_program_counter = *counter;
            },
            Instruction::Compare(comparison) => {
                let val1 = self.stack.pop()?;
                let val2 = self.stack.pop()?;
                let ret = comparison.perform_op(val1, val2)?;
                self.stack.push(ret);
            },
            Instruction::PerformOperation(operation) => {

            }
            Instruction::ConditionalJump(_) => {},
            Instruction::AddressOf => {},
            Instruction::Dereference => {},
            Instruction::Alloc => {},
            Instruction::Call(_) => {},
            Instruction::Throw => {},
            Instruction::Catch => {},
            Instruction::Pack => {},

        }

        Ok(())
    }
}

