
#[derive(Debug, PartialEq)]
pub enum Literal {
    Character(char),
    Boolean(bool),
    Integer(usize),
    Float(f64),
    String(String)
}


#[derive(Debug, PartialEq)]
pub enum Control {
    If,
    Else,
    Break,
    Return,
    Try,
    Catch,
    Case
}

#[derive(Debug, PartialEq)]
pub enum Loop {
    While,
    For,
    Do
}

#[derive(Debug, PartialEq)]
pub enum Security {
    Private,
    Public,
    Internal
}

#[derive(Debug, PartialEq)]
pub enum Primitive {
    Char,
    I8,
    I16,
    I32,
    I64,
    Imax,
    F64,
    F32
}

#[derive(Debug, PartialEq)]
pub enum Compound {
    Struct,
    Call,
    Trait,
    Enum
}

#[derive(Debug, PartialEq)]
pub enum Declare {
    Fn,
    Val,
    Var
}

#[derive(Debug, PartialEq)]
pub enum ObjectOrientation {
    This,
    Super,
    Abstract,
    Is,
    As
}

/// Used for structure
#[derive(Debug, PartialEq)]
pub enum Structural {
    /// ;
    Semicolon,
    /// {
    LCurl,
    /// }
    RCurl
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    /// !
    Bang,
    /// %
    Rem,
    /// &
    And,
    /// &&
    Dand,
    /// *
    Star,
    /// +
    Plus,
    /// ,
    Comma,
    /// -
    Minus,
    /// ->
    Arrow,
    /// .
    Dot,
    /// ...
    Ellipsis,
    /// /
    FwSlash,
    /// :
    Colon,
    /// <<
    LShift,
    /// >>
    RShift,
    /// =
    Assign,
    /// <
    Less,
    /// >
    Greater,
    /// <=
    LessEqual,
    /// >=
    GreaterEqual,
    /// ==
    Equal,
    /// !=,
    NEqual,
    /// ^
    Xor,
    /// |
    Bar,
    /// ||
    Or,
    /// ::
    Namespace,
    /// $
    Dollar,
    /// ?
    Qmark,
    /// (
    LPar,
    /// )
    RPar,
    /// [
    LBracket,
    /// ]
    RBracket
}