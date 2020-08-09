use crate::instruction_set::{Instruction, DataLocation, Size};
use crate::bytes::{InstructionBytes};
use crate::registers::Register;
use crate::InstructionBytesBuilder;


fn decompose_instruction(instruction: Instruction, builder: &mut InstructionBytesBuilder) -> InstructionBytes {
    unimplemented!();
    let opcode = instruction.get_opcode();
    builder.opcode(opcode);
    unsafe {
        match instruction {
            Instruction::PushVal(i) => {
                let val: usize = i.as_u64_no_coercion().into();
            },
            Instruction::Push(i, _) => {
                match i {
                    DataLocation::Location => {

                    },
                    DataLocation::Register(reg_type, index) => {
                        let reg: u8 = Register::from(reg_type, index).expect("Failed to get register from invalid instruction").into();

                    },
                    DataLocation::Immediate(imm) => {

                    },
                }
            },
            Instruction::Pop(i, _) => {
                match i {
                    DataLocation::Location => {

                    },
                    DataLocation::Register(reg_type, index) => {
                        let reg: u8 = Register::from(reg_type, index).expect("Failed to get register from invalid instruction").into();

                    },
                    DataLocation::Immediate(imm) => {

                    },
                }
            },
            Instruction::Ret(out) => {

            },
            Instruction::Jump(loc) => {

            },
            Instruction::Compare(op, _size, _prim_type) => {

            },
            Instruction::PerformOperation(_, _, _) => {},
            Instruction::ConditionalJump(_, _) => {},
            Instruction::AddressOf(_) => {},
            Instruction::Dereference(_) => {},
            Instruction::Call(_) => {},
            Instruction::Throw => {},
            Instruction::Catch => {},
            Instruction::Copy { .. } => {},
            Instruction::Move { .. } => {},
            Instruction::Nop => {},
            Instruction::Halt => {},
        }
    }
    builder.build()
}