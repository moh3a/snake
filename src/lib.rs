mod random;
mod snake;
use snake::SnakeGame;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    let mut game = SnakeGame::new(20, 20);

    todo!("setup the timer");
}
