mod utils;

use wasm_bindgen::prelude::*;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::HtmlTextAreaElement;
use js_sys::eval;



#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    // This allows us to call JavaScript's console.log() from Rust
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

#[wasm_bindgen]
pub fn eval_wasm(js_code: &str) -> Result<JsValue, JsValue> {
    eval(js_code)
}

#[wasm_bindgen]
pub fn create_element(element_name: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    document.create_element(element_name);
}

#[wasm_bindgen]
pub fn add_event_listener(element_id: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let input_element = document
        .get_element_by_id(element_id)
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();


    let closure = Closure::wrap(Box::new(|e: Event| {
        e.current_target()
            .unwrap()
            .dyn_into::<HtmlTextAreaElement>()
            .unwrap();

    }) as Box<dyn FnMut(_)>);

    input_element.add_event_listener_with_callback(element_id, &closure.as_ref().unchecked_ref());

    closure.forget();

}



#[wasm_bindgen]
pub fn log_to_console(message: &str) {
    log(message);
}

#[wasm_bindgen]
pub fn greet(greeting: &str) {
    alert(&format!("Hello, {}!", greeting));
}

#[wasm_bindgen]
pub fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    if n < 0 {
        return -1;
    }
    
    return fib(n - 1) + fib(n - 2);
}
