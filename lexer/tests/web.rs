//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use fluentd_lexer::{lex, lexer::Lexer, token::Token};

extern crate wasm_bindgen_test;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test::wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

macro_rules! literal {
    ($expr:expr) => {
        Token::Indetifier(String::from($expr).into())
    };
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn test_tokenize() {
    let input = "
<match pattern>
  @type forward
  <buffer>
    @type file
    path /path/to/buffer/forward
  </buffer>
</match>
"
    .to_string();


    let tokens = lex(input);
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
