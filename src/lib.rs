use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    let value = format!("Hello {}!", name);
    println!("{}", value);
    return value;
}
// wasm-pack build --target web

#[wasm_bindgen]
pub fn greet_alert(name: &str) {
    alert(name);
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}
