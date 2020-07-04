use crate::tokenization::{TokenType, Literal, Control, Token, Loop, Security, Compound, Declare, ObjectOrientation, Structural, Operator};
use std::str::Chars;
use std::error::Error;
use crate::tokenization::TokenType::EOF;
use std::fmt::{Display, Formatter};


pub struct Lexer{
    filename: String,
    string: Vec<char>,
    position: usize,
    current: Option<char>,
    line_number: usize,
    column: usize
}

#[derive(Debug)]
pub struct LexError(&'static str);

impl Display for LexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for LexError {

}

macro_rules! lerr {
    ($s:expr) => { return Err(LexError($s));};
}

impl Lexer{
    pub fn new(filename: String, input: String) -> Self {
        let mut output = Lexer {
            filename,
            string: input.chars().collect::<Vec<char>>(),
            position: 0,
            current: None,
            line_number: 1,
            column: 1
        };
        #[cfg(test)]
            {
                let take = output.take_current_char();
                assert!(take == None)
            }
        output.current = output.string.get(0).map(|c| *c);

        output
    }

    /// Returns the current char, or None if there is none
    fn current_char(&mut self) -> Option<char> {
        self.current
    }


    /// Gets the current char, then moves to the next character
    fn take_current_char(&mut self) -> Option<char> {
        let output = self.current;
        self.next_char();
        output
    }

    /// Moves the lexer up a character, and returns new current character
    fn next_char(&mut self) -> Option<char> {
        if self.current.is_some() {
            if self.current.unwrap() == '\n'{
                self.line_number += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.position += 1;


            self.current = self.string.get(self.position).map(|c| *c);

        }
        self.current
    }

    pub fn single_lex(&mut self) -> Result<Token, LexError> {
        while match self.current_char() {
            None => { return Ok(Token::new(EOF, self.filename.clone(), self.line_number, self.column)) },
            Some(c) => {
                c.is_whitespace()
            },
        } {
            self.next_char();
        }


        match self.current_char() {
            None => { Err(LexError("Illegal End of Token Stream")) },
            Some(current) => {
                let line = self.line_number;
                let column = self.column;

                if let Some(token_type) =
                match current {
                    'a'..='z' | 'A'..='Z' | '_' => {
                        let mut image = String::new();
                        while let Some(char) = self.current_char() {
                            if char.is_alphanumeric() {
                                image.push(char);
                            } else {
                                break;
                            }
                            self.next_char();
                        }

                        // Keywords here
                        match image.as_ref() {
                            "true" => Some(TokenType::Literal(Literal::Boolean(true))),
                            "false" => Some(TokenType::Literal(Literal::Boolean(false))),
                            "if" => Some(TokenType::Control(Control::If)),
                            "else" => Some(TokenType::Control(Control::Else)),
                            "break" => Some(TokenType::Control(Control::Break)),
                            "return" => Some(TokenType::Control(Control::Return)),
                            "try" => Some(TokenType::Control(Control::Try)),
                            "catch" => Some(TokenType::Control(Control::Catch)),
                            "case" => Some(TokenType::Control(Control::Case)),
                            "while" => Some(TokenType::Loop(Loop::While)),
                            "for" => Some(TokenType::Loop(Loop::For)),
                            "do" => Some(TokenType::Loop(Loop::Do)),
                            "private" => Some(TokenType::Security(Security::Private)),
                            "internal" => Some(TokenType::Security(Security::Internal)),
                            "public" => Some(TokenType::Security(Security::Public)),
                            "struct" => Some(TokenType::Compound(Compound::Struct)),
                            "call" => Some(TokenType::Compound(Compound::Call)),
                            "trait" => Some(TokenType::Compound(Compound::Trait)),
                            "enum" => Some(TokenType::Compound(Compound::Enum)),
                            "fn" => Some(TokenType::Declare(Declare::Fn)),
                            "val" => Some(TokenType::Declare(Declare::Val)),
                            "var" => Some(TokenType::Declare(Declare::Var)),
                            "this" => Some(TokenType::Object(ObjectOrientation::This)),
                            "super" => Some(TokenType::Object(ObjectOrientation::Super)),
                            "abstract" => Some(TokenType::Object(ObjectOrientation::Abstract)),
                            "is" => Some(TokenType::Object(ObjectOrientation::Is)),
                            "as" => Some(TokenType::Object(ObjectOrientation::As)),
                            _ => Some(TokenType::Identifier(image))
                        }


                    },
                    '0'..='9' => {
                        let mut base =0_usize;
                        while let Some(char) = self.current_char() {
                            if char.is_digit(10) {
                                let digit = char.to_digit(10).expect("Shouldn't fail, as checked");
                                base *= 10;
                                base += digit as usize;
                            } else if char.is_whitespace() || char == '.'{
                                break;
                            } else {
                                lerr!("Invalid number literal");
                            }
                            self.next_char();
                        }
                        if let Some(char) = self.current_char() {
                            if char == '.' {
                                let mut decimals = 0;
                                let mut base = base as f64;
                                while let Some(char) = self.current_char() {
                                    if char.is_digit(10) {
                                        let digit = char.to_digit(10).expect("Shouldn't fail, as checked");
                                        base *= 10.0;
                                        base += digit as f64;
                                        decimals += 1;
                                    } else if char.is_whitespace() {
                                        break;
                                    } else {
                                        lerr!("Invalid number literal");
                                    }
                                    self.next_char();
                                }
                                for _ in 0..decimals {
                                    base /= 10.0;
                                }
                                Some(TokenType::Literal(Literal::Float(base)))
                            } else {
                                Some(TokenType::Literal(Literal::Integer(base)))
                            }
                        } else {
                            None
                        }
                    },
                    '\'' => {
                        let c = self.next_char().expect("There must be a char after a '");
                        if self.next_char() != Some('\'') {
                            lerr!("There must be a following ' in a char literal")
                        }
                        Some(
                            TokenType::Literal(Literal::Character(c))
                        )
                    },
                    '"' => {
                        self.next_char();
                        let mut image = String::new();
                        loop {
                            match self.take_current_char() {
                                Some('"') => {
                                    break;
                                },
                                Some('\\') => {
                                    let op = self.take_current_char().expect("There must a be character following a \\ in a string");
                                    match op {
                                        'n' => {
                                            image.push('\n');
                                        },
                                        't' => {
                                            image.push('\t');
                                        }
                                        _ => {
                                            lerr!("Unsupported escape code")
                                        }
                                    }
                                },
                                Some('\n') => {
                                    lerr!("Strings must be on the same line")
                                }
                                Some(c) => {
                                    image.push(c);
                                },
                                None => {
                                    lerr!("Non-terminated String")
                                }
                            }
                        }
                        Some(TokenType::Identifier(image))
                    }
                    ';' => {
                        self.next_char();
                        Some(TokenType::Structural(Structural::Semicolon))
                    },
                    '{' => {
                        self.next_char();
                        Some(TokenType::Structural(Structural::LCurl))
                    },
                    '}' => {
                        self.next_char();
                        Some(TokenType::Structural(Structural::RCurl))
                    },
                    '=' => {
                        if self.next_char() == Some('=') {
                            self.next_char();
                            Some(TokenType::Operator(Operator::Equal))
                        } else {
                            Some(TokenType::Operator(Operator::Assign))
                        }
                    },
                    '!' => {
                        if self.next_char() == Some('=') {
                            self.next_char();
                            Some(TokenType::Operator(Operator::NEqual))
                        } else {
                            Some(TokenType::Operator(Operator::Bang))
                        }
                    },
                    '%' => {
                        if self.next_char() == Some('=') {
                            self.next_char();
                            Some(TokenType::CompoundAssignment(Operator::Rem))
                        } else {
                            Some(TokenType::Operator(Operator::Rem))
                        }
                    },
                    '&' => {
                        match self.next_char() {
                            Some('=') => {
                                self.next_char();
                                Some(TokenType::CompoundAssignment(Operator::And))
                            },
                            Some('&') => {
                                self.next_char();
                                Some(TokenType::Operator(Operator::Dand))
                            }
                            _ => {
                                Some(TokenType::Operator(Operator::And))
                            }
                        }
                    }
                    '*' => {
                        if self.next_char() == Some('=') {
                            self.next_char();
                            Some(TokenType::CompoundAssignment(Operator::Star))
                        } else {
                            Some(TokenType::Operator(Operator::Star))
                        }
                    },
                    '+' => {
                        if self.next_char() == Some('=') {
                            self.next_char();
                            Some(TokenType::CompoundAssignment(Operator::Plus))
                        } else {
                            Some(TokenType::Operator(Operator::Plus))
                        }
                    },
                    ',' => {
                        self.next_char();
                        Some(TokenType::Operator(Operator::Comma))
                    },
                    '-' => {
                        match self.next_char() {
                            Some('=') => {
                                self.next_char();
                                Some(TokenType::CompoundAssignment(Operator::Minus))
                            },
                            Some('>') => {
                                self.next_char();
                                Some(TokenType::Operator(Operator::Arrow))
                            }
                            _ => {
                                Some(TokenType::Operator(Operator::Minus))
                            }
                        }
                    },
                    '.' => {
                        if self.next_char() == Some('.') {
                            if self.next_char() == Some('.') {

                                Some(TokenType::Operator(Operator::Ellipsis))
                            } else {
                                lerr!(".. is not a valid operator, needs to be either . or ...")
                            }

                        } else {
                            Some(TokenType::Operator(Operator::Dot))
                        }
                    },
                    '/' => {
                        if self.next_char() == Some('=') {
                            self.next_char();
                            Some(TokenType::CompoundAssignment(Operator::FwSlash))
                        } else {
                            Some(TokenType::Operator(Operator::FwSlash))
                        }
                    },
                    ':' => {
                        if self.next_char() == Some(':') {
                            Some(TokenType::Operator(Operator::Namespace))
                        } else {
                            Some(TokenType::Operator(Operator::Colon))
                        }
                    },
                    '<' => {
                        match self.next_char() {
                            Some('=') => {
                                self.next_char();
                                Some(TokenType::Operator(Operator::LessEqual))
                            },
                            Some('<') => {
                                self.next_char();
                                Some(TokenType::Operator(Operator::LShift))
                            }
                            _ => {
                                Some(TokenType::Operator(Operator::Less))
                            }
                        }
                    },
                    '>' => {
                        match self.next_char() {
                            Some('=') => {
                                self.next_char();
                                Some(TokenType::Operator(Operator::GreaterEqual))
                            },
                            Some('>') => {
                                self.next_char();
                                Some(TokenType::Operator(Operator::LShift))
                            }
                            _ => {
                                Some(TokenType::Operator(Operator::Greater))
                            }
                        }
                    },
                    '^' => {
                        if self.next_char() == Some('=') {
                            self.next_char();
                            Some(TokenType::CompoundAssignment(Operator::Xor))
                        } else {
                            Some(TokenType::Operator(Operator::Xor))
                        }
                    },
                    '|' => {
                        match self.next_char() {
                            Some('=') => {
                                self.next_char();
                                Some(TokenType::CompoundAssignment(Operator::Bar))
                            },
                            Some('|') => {
                                self.next_char();
                                Some(TokenType::Operator(Operator::Or))
                            }
                            _ => {
                                Some(TokenType::Operator(Operator::Bar))
                            }
                        }
                    },
                    '$' => {
                        self.next_char();
                        Some(TokenType::Operator(Operator::Dollar))
                    },
                    '(' => {
                        self.next_char();
                        Some(TokenType::Operator(Operator::LPar))
                    },
                    ')' => {
                        self.next_char();
                        Some(TokenType::Operator(Operator::RPar))
                    },
                    '[' => {
                        self.next_char();
                        Some(TokenType::Operator(Operator::LBracket))
                    },
                    ']' => {
                        self.next_char();
                        Some(TokenType::Operator(Operator::RBracket))
                    },
                    _ => {
                        lerr!("Unsupported character")
                    }
                } {
                    Ok(Token::new(token_type, self.filename.clone(), line, column))
                } else {
                    lerr!("No token created!")
                }


            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lexing::Lexer;
    use crate::tokenization::{TokenType, Declare, HasTokenType};

    #[test]
    fn get_char() {
        let string = "a";
        let mut lexer = Lexer::new("test".to_string(), string.to_string());
        assert_eq!(lexer.current_char(), Some('a'))
    }

    #[test]
    fn lex_identifier() {
        let string = "   hello _hello3";
        let mut lexer = Lexer::new("test".to_string(), string.to_string());
        assert_eq!(lexer.single_lex().unwrap().token_type(), Some(&TokenType::Identifier("hello".to_string())));
        assert_eq!(lexer.single_lex().unwrap().token_type(), Some(&TokenType::Identifier("_hello3".to_string())))
    }

    #[test]
    #[should_panic]
    fn incorrect_number_fails() {
        let string = "3a";
        let mut lexer = Lexer::new("test".to_string(),string.to_string());
        lexer.single_lex();
    }

    #[test]
    fn lex() {
        let string = "var i: imax = 3.0 as imax;";
        let mut lexer = Lexer::new("test".to_string(),string.to_string());
        assert_eq!(lexer.single_lex().unwrap().token_type(), Some(&TokenType::Declare(Declare::Var)));
    }
}



