#![cfg(not(target_os = "android"))]

mod logic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logic::run_app()
}
