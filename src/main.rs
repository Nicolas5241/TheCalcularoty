#![windows_subsystem = "windows"]

use std::thread;

use rodio::{Decoder, OutputStreamBuilder, Sink};

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
	thread::spawn(|| {
		let file = std::fs::File::open("assets/music.mp3");
		if file.is_err() {
			return;
		}
		let file = unsafe { file.unwrap_unchecked() };

		let src = Decoder::new_looped(file);
		let stream_handle = OutputStreamBuilder::open_default_stream();

		if src.is_err() || stream_handle.is_err() {
			return;
		}

		let src = unsafe { src.unwrap_unchecked() };
		let stream_handle = unsafe { stream_handle.unwrap_unchecked() };
		
		let sink = Sink::connect_new(stream_handle.mixer());
		sink.append(src);
		sink.sleep_until_end();
	});

	logic::start_ui()
}
