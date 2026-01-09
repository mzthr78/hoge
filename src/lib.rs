use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() {
    log("uaaaaaa");

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let app = document.get_element_by_id("app").unwrap();
    let p = document.create_element("p").unwrap();
    p.set_inner_html("hogehogehoge");
    app.append_child(&p).unwrap();
}

#[wasm_bindgen]
pub fn add(left: u32, right: u32) -> u32{
    left + right
}
