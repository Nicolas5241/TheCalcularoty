#![windows_subsystem = "windows"]
#![no_std]

extern crate alloc;
use dlmalloc::GlobalDlmalloc;

#[global_allocator]
static ALLOCATOR: GlobalDlmalloc = GlobalDlmalloc;

mod logic;
mod units;
mod types;
mod utils;
mod calculations;
mod traits;
mod consts;
mod conversions;

fn main() -> Result<(), alloc::boxed::Box<dyn core::error::Error>> {
	logic::start_ui()
}
