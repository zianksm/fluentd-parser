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
        while self.peek() != '\0' {
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
                            panic!("Unknown identifier: {}", ident);
                        }
                    }
                }
            }
        }

        self.tokens.clone()
    }

    fn peek(&self) -> char {
        if self.pos + 1 < self.input.len() {
            self.input[self.pos + 1]
        } else {
            '\0'
        }
    }
    fn advance(&mut self) -> char {
        self.pos += 1;

        if self.pos < self.input.len() {
            self.current = self.input[self.pos];
        }

        self.current
    }

    fn skip_whitespace(&mut self) {
        while self.current.is_whitespace() {
            if self.current == '\n' {
                self.line += 1;
            }

            self.advance();
        }
    }

    fn parse_at_sign(&mut self) -> token::AtSignIdent {
        let ident = self.parse_until_non_ident();
        self.advance();
        let args = self.parse_until_non_ident();

        token::AtSignIdent::from_str_with_ident(ident, args).unwrap()
    }

    fn parse_until_non_ident(&mut self) -> String {
        let mut ident = vec![];

        while self.current.is_ascii_lowercase()
            || self.current.is_ascii_uppercase()
            || self.current.is_ascii_digit()
        {
            ident.push(self.current);
            self.advance();
        }

        ident.iter().collect::<String>()
    }

    fn parse_until_newline(&mut self) -> String {
        let mut ident = vec![];
        while self.current != '\n' {
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
    fn test_tokenize() {
        let file = "# Receive events from 24224/tcp
# This is used by log forwarding and the fluent-cat command
<source>
  @type forward
  port 24224
</source>";

        let mut lexer = Lexer::new(file.to_string());
        let tokens = lexer.tokenize();

        println!("{:?}", tokens);

        assert_eq!(
            tokens[0],
            Token::HashTag("Receive events from 24224/tcp".to_string())
        );
        assert_eq!(
            tokens[1],
            Token::HashTag(
                "This is used by log forwarding and the fluent-cat command".to_string()
            )
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
}
