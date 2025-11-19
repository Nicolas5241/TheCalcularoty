#![windows_subsystem = "windows"]

use std::error::Error;

mod not_android;
mod units;
mod types;
mod utils;
mod calculations;
mod traits;
mod consts;
mod conversions;

pub fn main() -> Result<(), Box<dyn Error>> {
    not_android::run_app()
}
