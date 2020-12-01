#![recursion_limit = "512"]

mod ensemble;
mod nav_bar;
mod utils;
mod views;

use ensemble::*;
use log::*;
use wasm_bindgen::prelude::*;
use yew::*;

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_log::init_with_level(Level::Debug).expect("failed to initialize logger");
    App::<Ensemble>::new().mount_to_body();
    info!("App mounted.");
    Ok(())
}
