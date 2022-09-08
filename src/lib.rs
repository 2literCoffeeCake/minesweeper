mod mines;
mod ui;
mod browser_util;

use browser_util::{get_item_from_client_storage, get_body};
use ui::App;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    yew::start_app::<App>();
    update_body();
    Ok(())
}

fn update_body(){
    let schema = get_item_from_client_storage("color_schema");
    if let Some(schema) = schema {
        let body = get_body().unwrap();
        body.set_class_name(&schema);
    } 
}