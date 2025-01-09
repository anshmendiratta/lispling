#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![feature(slice_as_array)]

#[derive(Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    Numeric(Number),
    Whitespace,
    Op(Operator),
}

#[derive(Debug)]
pub enum Number {
    // TOOD: Add support for later.
    // FloatingPoint(f64),
    Integer(isize),
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Asterisk,
    // /
    Slash,
}

struct Expression {
    operands: (Number, Number),
    operator: Operator,
}

pub struct Lexer<'a> {
    src: &'a str,
    current_token: Option<char>,
    rest: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(expr_as_string: &'a str) -> Self {
        Self {
            src: expr_as_string,
            current_token: None,
            rest: expr_as_string,
        }
    }

    pub fn lex(mut self) -> Vec<Token> {
        let mut tokens = vec![];

        self.advance();
        while self.current_token.is_some() {
            match self.tokenize_character(self.current_token.unwrap()) {
                Some(t) => tokens.push(t),
                _ => {
                    let column = self.src.find(self.current_token.unwrap()).unwrap();
                    panic!(
                        "Unexpected token: {:?} at col {} ",
                        self.current_token.unwrap(),
                        column
                    );
                }
            };

            self.advance();
        }

        tokens
    }

    fn tokenize_character(&mut self, character: char) -> Option<Token> {
        match character {
            '(' => return Some(Token::LeftParen),
            ')' => return Some(Token::RightParen),
            '0'..'9' => {
                return Some(Token::Numeric(Number::Integer(
                    character.to_digit(10).unwrap() as isize,
                )))
            }
            '+' => return Some(Token::Op(Operator::Plus)),
            '-' => return Some(Token::Op(Operator::Minus)),
            '*' => return Some(Token::Op(Operator::Asterisk)),
            '/' => return Some(Token::Op(Operator::Slash)),
            ' ' => return Some(Token::Whitespace),
            _ => None,
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.rest.as_bytes().len() >= 1 {
            self.current_token = Some(self.rest.bytes().nth(0).unwrap() as char);
            self.rest = &self.rest[1..];
            return self.current_token;
        } else {
            self.current_token = None;
            return None;
        }
    }

    // fn tokenize(self) -> ASTNode {}
}
