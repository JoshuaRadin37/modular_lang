mod subtypes;

pub use subtypes::*;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Identifier(String),
    Literal(Literal),
    Control(Control),
    Loop(Loop),
    Security(Security),
    Primitive(Primitive),
    Compound(Compound),
    Declare(Declare),
    Object(ObjectOrientation),
    Structural(Structural),
    Operator(Operator),
    CompoundAssignment(Operator),
    EOF,
}

pub struct Token {
    token_type: TokenType,
    filename: String,
    line_number: usize,
    column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, filename: String, line_number: usize, column: usize) -> Self {
        Self {
            token_type,
            filename,
            line_number,
            column,
        }
    }

    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }
}

pub trait HasTokenType {
    fn token_type(&self) -> Option<&TokenType>;
}

impl HasTokenType for Token {
    fn token_type(&self) -> Option<&TokenType> {
        Some(&self.token_type)
    }
}

impl HasTokenType for Option<Token> {
    fn token_type(&self) -> Option<&TokenType> {
        match self {
            None => None,
            Some(tok) => tok.token_type(),
        }
    }
}
