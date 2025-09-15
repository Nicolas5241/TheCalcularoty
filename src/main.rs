#![cfg(not(target_os = "android"))]

mod not_android;
mod units;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    not_android::run_app()
}
