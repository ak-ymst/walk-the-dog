use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

#[macro_use]
mod browser;
mod engine;
mod game;
mod segment;
mod sound;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    log!("Hello World 2");

    browser::spawn_local(async move {
        let game = game::WalkTheDog::new();

        engine::GameLoop::start(game)
            .await
            .expect("Could not start game loop");
    });

    Ok(())
}
