use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module="/www/utils/rand.ts")]
extern {
    pub fn rand(max: usize) -> usize;
}
