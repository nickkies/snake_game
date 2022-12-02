use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet(num: i32) {
    alert(num);
}

#[wasm_bindgen]
extern {
  pub fn alert(num: i32);
}

// wasm-pack build --target web
