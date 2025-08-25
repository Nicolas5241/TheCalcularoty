use std::thread;
use rodio::{Sink, OutputStreamBuilder, Decoder};

slint::include_modules!();

pub fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    thread::spawn(||{
        let stream_handle = OutputStreamBuilder::open_default_stream().unwrap();
        let sink = Sink::connect_new(stream_handle.mixer());

        let file = std::fs::File::open("res/music.mp3").unwrap();
        sink.append(Decoder::try_from(file).unwrap());
    });
    Ok(())
}
