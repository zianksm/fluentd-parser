use crate::lexer::token::{ArbitraryArgs, ArbitraryIdent};

use self::token::Token;

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
        while !self.is_at_end() {
            let current = self.current;
            println!("current: {}", current);

            self.skip_whitespace();

            match self.current {
                '@' => {
                    self.advance();
                    let args = self.parse_at_sign();
                    self.tokens.push(Token::AtSign(args));
                }
                '#' => {
                    self.advance();
                    let comments_literal = self.parse_until_newline();
                    self.tokens
                        .push(Token::HashTag(comments_literal.trim().to_string()));
                }
                '"' => {
                    self.advance();
                    self.tokens.push(Token::Quote);
                }
                '/' => {
                    self.advance();
                    self.tokens.push(Token::ForwardSlash);
                }
                '<' => {
                    self.advance();
                    self.tokens.push(Token::LeftAngle);
                }
                '>' => {
                    self.advance();
                    self.tokens.push(Token::RightAngle);
                }
                _ => {
                    let ident = self.parse_until_non_ident();
                    match ident.as_str() {
                        "port" => {
                            self.advance();
                            let port = self.parse_until_non_ident();
                            self.tokens.push(Token::Port(port.parse::<u16>().unwrap()));
                        }
                        "source" => {
                            self.tokens.push(Token::Source);
                        }
                        "match" => {
                            self.tokens.push(Token::Match);
                        }
                        "filter" => {
                            self.tokens.push(Token::Filter);
                        }
                        "system" => {
                            self.tokens.push(Token::System);
                        }
                        "label" => {
                            self.tokens.push(Token::Label);
                        }
                        "worker" => {
                            self.tokens.push(Token::Worker);
                        }
                        _ => {
                            self.advance();
                            let args = self.parse_until_whitespace();
                            self.tokens
                                .push(Token::Ident(ArbitraryIdent(ident), ArbitraryArgs(args)))
                        }
                    }
                }
            }
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

    #[inline]
    fn skip_whitespace(&mut self) {
        while self.current.is_whitespace() {
            if self.current == '\n' {
                self.line += 1;
            }

            self.advance();
        }
    }

    #[inline]
    fn parse_at_sign(&mut self) -> token::AtSignIdent {
        let ident = self.parse_until_non_ident();
        self.advance();
        let args = self.parse_until_non_ident();

        token::AtSignIdent::from_str_with_ident(ident, args).unwrap()
    }

    #[inline]
    fn parse_until_non_ident(&mut self) -> String {
        let mut ident = vec![];

        while self.is_ident() {
            ident.push(self.current);

            if self.is_at_end() {
                break;
            }

            self.advance();
        }

        ident.iter().collect::<String>()
    }

    #[inline]
    fn is_ident(&mut self) -> bool {
        self.current.is_ascii_lowercase()
            || self.current.is_ascii_uppercase()
            || self.current.is_ascii_digit()
    }

    #[inline]
    fn parse_until_newline(&mut self) -> String {
        let mut ident = vec![];
        while self.current != '\n' {
            ident.push(self.current);
            self.advance();
        }

        ident.iter().collect::<String>()
    }

    #[inline]
    fn parse_until_whitespace(&mut self) -> String {
        let mut ident = vec![];
        while !self.current.is_whitespace() && !self.is_at_end() {
            ident.push(self.current);
            self.advance();
        }

        ident.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_until_whitespace() {
        let mut lexer = Lexer::new("hello world".to_string());
        let result = lexer.parse_until_whitespace();

        assert_eq!(result, "hello");
    }

    #[test]
    fn test_peek() {
        let file = "test";
        let mut lexer = Lexer::new(file.to_string());

        assert_eq!(lexer.peek(), 'e');
        lexer.advance();
        assert_eq!(lexer.peek(), 's');
        lexer.advance();
        assert_eq!(lexer.peek(), 't');
        lexer.advance();
        assert_eq!(lexer.peek(), '\0');
        assert_eq!(lexer.current, 't');
    }

    #[test]
    fn test_advance() {
        let file = "test";
        let mut lexer = Lexer::new(file.to_string());

        assert_eq!(lexer.advance(), 'e');
        assert_eq!(lexer.advance(), 's');
        assert_eq!(lexer.advance(), 't');
        assert_eq!(lexer.advance(), '\0');
    }

    #[test]
    fn test_variable_declration() {
        let file = "port 24224";
        let mut lexer = Lexer::new(file.to_string());
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Port(24224));
    }

    #[test]
    fn test_at_sign() {
        let file = "@type forward";
        let mut lexer = Lexer::new(file.to_string());
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens[0],
            Token::AtSign(token::AtSignIdent::Type("forward".to_string()))
        );
    }

    #[test]
    fn test_tokenize() {
        let file = "# Receive events from 24224/tcp
# This is used by log forwarding and the fluent-cat command
<source>
  @type forward
  port 24224
</source>
";

        let mut lexer = Lexer::new(file.to_string());
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens[0],
            Token::HashTag("Receive events from 24224/tcp".to_string())
        );
        assert_eq!(
            tokens[1],
            Token::HashTag("This is used by log forwarding and the fluent-cat command".to_string())
        );
        assert_eq!(tokens[2], Token::LeftAngle);
        assert_eq!(tokens[3], Token::Source);
        assert_eq!(tokens[4], Token::RightAngle);
        assert_eq!(
            tokens[5],
            Token::AtSign(token::AtSignIdent::Type("forward".to_string()))
        );
        assert_eq!(tokens[6], Token::Port(24224));
        assert_eq!(tokens[7], Token::LeftAngle);
    }

    #[test]
    fn test_arbitrary_identifier() {
        let file = "abc";
        let mut lexer = Lexer::new(file.to_string());

        let tokens = lexer.tokenize();

        assert_eq!(
            tokens[0],
            Token::Ident(
                ArbitraryIdent("abc".to_string()),
                ArbitraryArgs("".to_string())
            )
        );
    }

    #[test]
    fn test_parse_until_newline() {
        let file = "# Receive events from 24224/tcp\n";
        let mut lexer = Lexer::new(file.to_string());
        let result = lexer.parse_until_newline();

        assert_eq!(result, "# Receive events from 24224/tcp");
    }

    #[test]
    fn test_parse_at_sign() {
        let file = "@type forward";
        let mut lexer = Lexer::new(file.to_string());
        lexer.advance();
        let result = lexer.parse_at_sign();

        if let token::AtSignIdent::Type(ident) = result {
            assert_eq!(ident, "forward");
        } else {
            panic!("Invalid result: {:?}", result);
        }
    }

    #[test]
    fn test_parse_until_non_ident() {
        let file = "port 24224";
        let mut lexer = Lexer::new(file.to_string());
        let result = lexer.parse_until_non_ident();

        assert_eq!(result, "port");
    }

    #[test]
    fn test_skip_whitespace() {
        let file = "  \n  \t  \n  port 24224";
        let mut lexer = Lexer::new(file.to_string());
        lexer.skip_whitespace();

        assert_eq!(lexer.current, 'p');
    }
}
