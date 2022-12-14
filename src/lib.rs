pub mod window;
pub mod view;
mod index;

use window::window::*;
use view::size::{Size, Edge};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    let w = Window::new(index::Login::new()).expect("Failed to init window");
    w.build().expect("Failed to build window");
}
