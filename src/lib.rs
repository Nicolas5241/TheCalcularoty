#![cfg(target_os = "android")]

mod logic;
mod utils;
mod types;
mod units;
mod traits;
mod calculations;
mod consts;
mod conversions;

#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn std::error::Error>> {
    slint::android::init(app.clone())?;

    //android::run_app(app.asset_manager())
	logic::start_ui()
}
