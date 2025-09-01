#![cfg(not(target_os = "android"))]

mod not_android;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    not_android::run_app()
}
