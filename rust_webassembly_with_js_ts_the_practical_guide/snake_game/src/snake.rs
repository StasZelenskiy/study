use wasm_bindgen::prelude::wasm_bindgen;
use crate::direction::Direction;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell(pub usize);

pub(crate) struct Snake {
    pub(crate) body: Vec<SnakeCell>,
    pub(crate) direction: Direction,
}

impl Snake {
    pub(crate) fn new(spawn: usize, size: usize) -> Self {
        let body = (0..size).map(|i| SnakeCell(spawn - i)).collect();
        Self {
            body,
            direction: Direction::Right,
        }
    }
}
