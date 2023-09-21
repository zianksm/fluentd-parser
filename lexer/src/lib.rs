
#![cfg_attr(not(feature = "std"), no_std)]

mod utils;
mod lexer;
extern crate alloc;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, parser!");
}
