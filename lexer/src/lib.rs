#![cfg_attr(not(feature = "std"), no_std)]

pub mod lexer;
mod utils;
extern crate alloc;

use lexer::token::Token;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;


#[cfg(feature = "js")]
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    set_panic_hook();
    Ok(())
}

#[cfg(feature = "js")]
#[wasm_bindgen]
pub fn lex(input: &str) -> JsValue {
    let mut lexer = lexer::Lexer::new(input.to_string());
    let tokens = lexer.tokenize();

    JsValue::from_serde::<Vec<Token>>(&tokens).unwrap()
}
