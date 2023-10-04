mod utils;

use wasm_bindgen::prelude::*;
fn main() {
    token::lexer::token::Literal
}
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, parser!");
}
