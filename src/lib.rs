use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    let value = format!("Hello {}!", name);
    println!("{}", value);
    return value;
}

#[wasm_bindgen]
pub fn add(n1: f64, n2: f64) -> f64 {
    return n1 + n2;
}

#[wasm_bindgen]
pub fn greet_alert(name: &str) {
    alert(name);
}

#[wasm_bindgen]
pub fn get_now() -> usize {
    now()
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen(module = "/www/utils/date.js")]
extern "C" {
    fn now() -> usize;
}
