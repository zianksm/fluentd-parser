use crate::lexer::token::Literal;

use self::token::{ Token, TokenTypeStateMarker };

pub mod token;

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
        while !self.is_at_end() {
            match self.current {
                '.' => self.tokens.push(Token::Dot),
                '\n' => {
                    self.tokens.push(Token::Newline);
                    self.line += 1;
                }
                ' ' => self.tokens.push(Token::Whitespace),
                '<' => self.tokens.push(Token::LeftAngle),
                '>' => self.tokens.push(Token::RightAngle),
                '{' => self.tokens.push(Token::LeftCurly),
                '}' => self.tokens.push(Token::RightCurly),
                '@' => self.tokens.push(Token::AtSign),
                '#' => self.tokens.push(Token::HashTag),
                '"' => self.tokens.push(Token::Quote),
                '/' => self.tokens.push(Token::ForwardSlash),
                '\\' => self.tokens.push(Token::BackSlash),
                '(' => self.tokens.push(Token::LeftParen),
                ')' => self.tokens.push(Token::RightParen),
                '[' => self.tokens.push(Token::LeftBracket),
                ']' => self.tokens.push(Token::RightBracket),
                ',' => self.tokens.push(Token::Comma),
                ';' => self.tokens.push(Token::Semicolon),
                ':' => self.tokens.push(Token::Colon),
                '=' => self.tokens.push(Token::Equals),
                _ => {
                    let ident = self.parse_ident();
                    self.tokens.push(ident);
                }
            }

            // reached eof dont advance
            if self.pos + 1 >= self.input.len() {
                break;
            }

            self.advance();
        }

        self.tokens.clone()
    }

    #[inline]
    fn is_at_end(&self) -> bool {
        self.pos >= self.input.len()
    }

    #[inline]
    fn peek(&self) -> char {
        if self.pos + 1 < self.input.len() { self.input[self.pos + 1] } else { '\0' }
    }

    #[inline]
    fn advance(&mut self) -> char {
        if self.pos + 1 >= self.input.len() {
            return self.current;
        }

        self.pos += 1;
        self.current = self.input[self.pos];

        self.current
    }

    fn parse_ident(&mut self) -> Token {
        let mut ident = String::new();

        while Token::is_ident(&self.current) {
            ident.push(self.current);

            if Token::is_ident(&self.peek()) {
                self.advance();
            } else {
                break;
            }
        }

        Token::infer_keyword(ident.as_str())
    }
}

#[cfg(test)]
mod tests {
    macro_rules! literal {
        ($expr:expr) => {
            Token::Indetifier(String::from($expr).into())
        };
    }
    use super::*;

    #[test]
    fn test_tokenize() {
        let input =
            "
<match pattern>
  @type forward
  <buffer>
    @type file
    path /path/to/buffer/forward
  </buffer>
</match>
".to_string();

        println!("{:?}\n\n", input);
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        println!("{:?}", tokens);

        assert_eq!(tokens[0], Token::Newline);

        assert_eq!(tokens[1], Token::LeftAngle);
        assert_eq!(tokens[2], Token::Match);
        assert_eq!(tokens[3], Token::Whitespace);
        assert_eq!(tokens[4], literal!("pattern"));
        assert_eq!(tokens[5], Token::RightAngle);
        assert_eq!(tokens[6], Token::Newline);

        assert_eq!(tokens[7], Token::Whitespace);
        assert_eq!(tokens[8], Token::Whitespace);
        assert_eq!(tokens[9], Token::AtSign);
        assert_eq!(tokens[10], literal!("type"));
        assert_eq!(tokens[11], Token::Whitespace);
        assert_eq!(tokens[12], literal!("forward"));
        assert_eq!(tokens[13], Token::Newline);

        assert_eq!(tokens[14], Token::Whitespace);
        assert_eq!(tokens[15], Token::Whitespace);
        assert_eq!(tokens[16], Token::LeftAngle);
        assert_eq!(tokens[17], Token::Buffer);
        assert_eq!(tokens[18], Token::RightAngle);
        assert_eq!(tokens[19], Token::Newline);

        assert_eq!(tokens[20], Token::Whitespace);
        assert_eq!(tokens[21], Token::Whitespace);
        assert_eq!(tokens[22], Token::Whitespace);
        assert_eq!(tokens[23], Token::Whitespace);
        assert_eq!(tokens[24], Token::AtSign);
        assert_eq!(tokens[25], literal!("type"));
        assert_eq!(tokens[26], Token::Whitespace);
        assert_eq!(tokens[27], literal!("file"));
        assert_eq!(tokens[28], Token::Newline);

        assert_eq!(tokens[29], Token::Whitespace);
        assert_eq!(tokens[30], Token::Whitespace);
        assert_eq!(tokens[31], Token::Whitespace);
        assert_eq!(tokens[32], Token::Whitespace);
        assert_eq!(tokens[33], literal!("path"));
        assert_eq!(tokens[34], Token::Whitespace);
        assert_eq!(tokens[35], Token::ForwardSlash);
        assert_eq!(tokens[36], literal!("path"));
        assert_eq!(tokens[37], Token::ForwardSlash);
        assert_eq!(tokens[38], literal!("to"));
        assert_eq!(tokens[39], Token::ForwardSlash);
        // registered as a keyword, will be resolved at parser level
        assert_eq!(tokens[40], Token::Buffer);
        assert_eq!(tokens[41], Token::ForwardSlash);
        assert_eq!(tokens[42], literal!("forward"));
        assert_eq!(tokens[43], Token::Newline);

        assert_eq!(tokens[44], Token::Whitespace);
        assert_eq!(tokens[45], Token::Whitespace);
        assert_eq!(tokens[46], Token::LeftAngle);
        assert_eq!(tokens[47], Token::ForwardSlash);
        assert_eq!(tokens[48], Token::Buffer);
        assert_eq!(tokens[49], Token::RightAngle);
        assert_eq!(tokens[50], Token::Newline);

        assert_eq!(tokens[51], Token::LeftAngle);
        assert_eq!(tokens[52], Token::ForwardSlash);
        assert_eq!(tokens[53], Token::Match);
        assert_eq!(tokens[54], Token::RightAngle);
        assert_eq!(tokens[55], Token::Newline);
    }
}
