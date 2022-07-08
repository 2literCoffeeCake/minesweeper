

mod mines;
mod ui;

use ui::App;


extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    yew::start_app::<App>();
    Ok(())
}

