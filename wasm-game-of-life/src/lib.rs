mod utils;

use wasm_bindgen::prelude::*;
//use web_sys::console;


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}


#[wasm_bindgen]
extern "C" {
    // This allows us to call JavaScript's console.log() from Rust
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
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
