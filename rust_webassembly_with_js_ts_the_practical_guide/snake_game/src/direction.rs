use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
