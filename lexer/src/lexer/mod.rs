use crate::lexer::token::{ Literal, Literal, PortNumber, Literal };

use self::token::{ Token, TokenTypeStateMarker };

mod token;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    current: char,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let input = input.chars().collect::<Vec<char>>();
        let pos = 0;
        let line = 1;
        let current = input[pos];

        Self {
            input,
            pos,
            line,
            current,
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        while !self.is_at_end() {}

        self.tokens.clone()
    }

    #[inline]
    fn is_at_end(&self) -> bool {
        self.peek() == '\0'
    }

    #[inline]
    fn peek(&self) -> char {
        if self.pos + 1 < self.input.len() { self.input[self.pos + 1] } else { '\0' }
    }

    #[inline]
    fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.pos += 1;
        self.current = self.input[self.pos];

        self.current
    }
}
