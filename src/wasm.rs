#[path = "logic.rs"]
mod logic;

pub fn run_app() -> Result<(), Box<dyn std::error::Error>> {
	logic::start_ui()
}
