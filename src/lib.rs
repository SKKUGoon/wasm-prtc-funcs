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

#[wasm_bindgen]
pub struct Foo {
    value: String,
}

#[wasm_bindgen]
impl Foo {
    pub fn new(value: &str) -> Foo {
        Foo {
            value: String::from(value),
        }
    }

    pub fn bar(&self) -> String {
        format!("Hello from Rust WASM! You've entered: {}", self.value)
    }
}
