#![cfg(any(target_os = "android", target_family = "wasm"))]

#[cfg(target_os = "android")]
mod android;

#[cfg(target_family = "wasm")]
mod wasm;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

mod utils;
mod types;
mod units;
mod traits;
mod calculations;
mod consts;
mod conversions;

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn std::error::Error>> {
    slint::android::init(app.clone())?;

    android::run_app(app.asset_manager())
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen(start)]
pub fn main() {
	console_error_panic_hook::set_once();
	wasm::run_app().expect("error starting event loop")
}
