use crate::resolution::{FullIdentifier, Identifier, Resolvable};
use crate::instruction_set::{Immediate, Instruction};

#[derive(Clone, Debug)]
pub struct Function {
    identifier: FullIdentifier,
    parameters: Vec<(Identifier, Immediate)>,
    ret_type: Option<Box<Immediate>>,
    instructions: Vec<Instruction>
}

impl Function {

    pub fn get_parameters(&self) -> &Vec<(Identifier, Immediate)> {
        &self.parameters
    }

    pub fn get_ret_type(&self) -> Option<&Immediate> {
        match &self.ret_type {
            None => None,
            Some(ret) => {
                Some(&*ret)
            },
        }
    }

    pub fn get_instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }
}

pub struct FunctionBuilder {
    in_progress: Function,
    set_parameters: bool,
    set_instructions: bool
}

impl FunctionBuilder {

    pub fn with_name(name: FullIdentifier) -> Self {
        FunctionBuilder {
            in_progress: Function {
                identifier: name,
                parameters: vec![],
                ret_type: Option::None,
                instructions: vec![]
            },
            set_parameters: false,
            set_instructions: false
        }
    }

    pub fn no_parameters(mut self) -> Self {
        if self.set_parameters {
            panic!("Parameters for {} already set", self.in_progress.identifier);
        }
        self.set_parameters = true;
        self
    }

    pub fn with_parameters(mut self, parameters: Vec<(Identifier, Immediate)>) -> Self {
        if self.set_parameters {
            panic!("Parameters for {} already set", self.in_progress.identifier);
        }
        self.in_progress.parameters = parameters;
        self.set_parameters = true;
        self
    }

    pub fn with_instructions(mut self, instructions: Vec<Instruction>) -> Self {
        if self.set_instructions {
            panic!("Instructions for {} already set", self.in_progress.identifier);
        }
        self.in_progress.instructions = instructions;
        self.set_instructions = true;
        self
    }

    pub fn build(self) -> Function {
        if !(self.set_instructions && self.set_parameters) {
            panic!("Trying to build unfinished function {}", self.in_progress.identifier);
        }

        self.in_progress
    }
}

impl Resolvable for Function {
    fn get_identifier(&self) -> &FullIdentifier {
        &self.identifier
    }
}

#[cfg(test)]
mod test {
    use crate::resolution::functions::FunctionBuilder;
    use crate::resolution::{FullIdentifier, Resolvable};

    #[test]
    fn can_build() {

        let function =
            FunctionBuilder::with_name(FullIdentifier::from("get"))
                .no_parameters()
                .with_instructions(vec![])
                .build();

        assert_eq!(format!("{}", function.get_identifier()), "get");
    }

    #[test]
    #[should_panic]
    fn panic_if_invalid_build() {
        FunctionBuilder::with_name(FullIdentifier::from("get"))
            .no_parameters()
            .build();
    }
}



