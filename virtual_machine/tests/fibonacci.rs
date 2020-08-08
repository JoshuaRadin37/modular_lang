use virtual_machine::instruction_set::Immediate::U32;
use virtual_machine::instruction_set::Instruction::*;
use virtual_machine::instruction_set::Literal;
use virtual_machine::instruction_set::RegisterType::{Callee, Caller};
use virtual_machine::instruction_set::{Immediate, JumpType, Operation};
use virtual_machine::memory::Scope::Local;
use virtual_machine::vm::VirtualMachine;

fn fib(n: usize) -> u32 {
    match n {
        0 | 1 => n as u32,
        _ => fib(n - 1) + fib(n - 2),
    }
}

#[test]
fn fib_test() {
    for n in 0..16 {
        let instructions = vec![
            Enter,
            DeclareVar("n".to_string(), Local),
            Push {
                src: Literal::Register(Callee, 0),
            },
            SaveVar("n".to_string()),
            PushVal(Immediate::USize(2)),
            GetVar("n".to_string()),
            PerformOperation(Operation::Subtract),
            ConditionalJump(JumpType::Below, 41 - 13),
            Push {
                src: Literal::Register(Callee, 0),
            },
            PushVal(Immediate::USize(2)),
            GetVar("n".to_string()),
            PerformOperation(Operation::Subtract),
            PopTo(Literal::Register(Callee, 0)),
            Push {
                src: Literal::Register(Callee, 1),
            },
            Call(0),
            PopTo(Literal::Register(Callee, 1)),
            PushVal(Immediate::USize(1)),
            GetVar("n".to_string()),
            PerformOperation(Operation::Subtract),
            PopTo(Literal::Register(Callee, 0)),
            Call(0),
            Push {
                src: Literal::Register(Callee, 1),
            },
            PerformOperation(Operation::Add),
            PopTo(Literal::Register(Caller, 0)),
            PopTo(Literal::Register(Callee, 1)),
            PopTo(Literal::Register(Callee, 0)),
            Exit,
            Ret(Some(Literal::Register(Caller, 0))),
            Exit,
            Move {
                dest: Literal::Register(Caller, 0),
                src: Literal::Register(Callee, 0),
            },
            Ret(Some(Literal::Register(Caller, 0))),
            Nop,
            Nop,
            Nop,
            Nop,
            Move {
                dest: Literal::Register(Callee, 0),
                src: Literal::Immediate(Immediate::USize(n)),
            },
            Call(0),
            Coerce { dest_type: U32(0) },
            Halt,
        ];

        let result = VirtualMachine::execute(instructions, 46 - 13);
        println!("Result = {:?}", result);
        assert_eq!(result.unwrap(), fib(n));
    }
}
