use crate::lexer::token::Literal;

use self::token::{Token, TokenTypeStateMarker};

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
        for token in self.input.clone().iter_mut() {
            match token {
                '.' => self.tokens.push(Token::Dot),
                '\n' => self.tokens.push(Token::Newline),
                ' ' => self.tokens.push(Token::Whitespace),
                '<' => self.tokens.push(Token::LeftAngle),
                '>' => self.tokens.push(Token::RightAngle),
                '{' => self.tokens.push(Token::LeftCurly),
                '}' => self.tokens.push(Token::RightCurly),
                '@' => self.tokens.push(Token::AtSign),
                '#' => self.tokens.push(Token::HashTag),
                '"' => self.tokens.push(Token::Quote),
                '/' => self.tokens.push(Token::ForwardSlash),
                _ => {
                    let ident = self.parse_ident();
                }
            }

            self.advance();
        }

        self.tokens.clone()
    }

    #[inline]
    fn is_at_end(&self) -> bool {
        self.peek() == '\0'
    }

    #[inline]
    fn peek(&self) -> char {
        if self.pos + 1 < self.input.len() {
            self.input[self.pos + 1]
        } else {
            '\0'
        }
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

    fn parse_ident(&mut self) -> Token {
        let mut ident = String::new();

        while Token::is_ident(&self.current) {
            ident.push(self.current);
            self.advance();
        }

        Token::infer_keyword(ident.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "<match pattern>
  @type forward
  <buffer>
    @type file
    path /path/to/buffer/forward
  </buffer>
</match>"
            .to_string();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 14);
        assert_eq!(tokens[0], Token::LeftAngle);
        assert_eq!(tokens[1], Token::Indetifier("match".to_string().into()));
        assert_eq!(tokens[2], Token::Whitespace);
        assert_eq!(tokens[3], Token::Indetifier("pattern".to_string().into()));
        assert_eq!(tokens[4], Token::RightAngle);
        assert_eq!(tokens[5], Token::Newline);
        assert_eq!(tokens[6], Token::Whitespace);
        assert_eq!(tokens[7], Token::AtSign);
        assert_eq!(tokens[8], Token::Indetifier("type".to_string().into()));
        assert_eq!(tokens[9], Token::Whitespace);
        assert_eq!(tokens[10], Token::Indetifier("forward".to_string().into()));
        assert_eq!(tokens[11], Token::Newline);
        assert_eq!(tokens[12], Token::Whitespace);
        assert_eq!(tokens[13], Token::RightAngle);
    }
}
