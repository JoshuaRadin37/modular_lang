use crate::resolution::Identifier;

pub trait Simplifier {
    type Input;
    type Output;

    fn simplify(&self, input: Self::Input) -> Self::Output;
    fn unsimplified(&self, output: Self::Output) -> Self::Input;
}

pub struct TupleMember;

impl Simplifier for TupleMember {
    type Input = usize;
    type Output = Identifier;

    fn simplify(&self, input: Self::Input) -> Self::Output {
        Identifier::new(format!("_{}", input)).unwrap()
    }

    fn unsimplified(&self, output: Self::Output) -> Self::Input {
        output.as_ref().parse().unwrap()
    }
}
