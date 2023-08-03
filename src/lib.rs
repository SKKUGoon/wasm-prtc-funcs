use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process_json(input_json: &str) -> String {
    let output_json = format!("message is {}", input_json);
    output_json
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}", name));
}
