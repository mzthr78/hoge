use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() {
    log("Hello Wasm");
}

/*
#[wasm_bindgen]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
*/
