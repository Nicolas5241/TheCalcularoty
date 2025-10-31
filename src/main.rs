#![cfg(not(target_os = "android"))]

#![windows_subsystem = "windows"]

mod logic;
//mod not_android;
mod units;
mod types;
mod utils;
mod calculations;
mod traits;
mod consts;
mod conversions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //not_android::run_app()
	logic::start_ui()
}
