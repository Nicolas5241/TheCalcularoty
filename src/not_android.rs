use std::error::Error;
use rodio::{OutputStreamBuilder, Sink, Decoder};

#[path = "logic.rs"]
mod logic;

pub fn run_app() -> Result<(), Box<dyn Error>> {
    std::thread::spawn(||{
        let _ = play_background_music();
    });

    logic::start_ui()
}

fn play_background_music() -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open("assets/music.mp3")?;

    let source = Decoder::new_looped(file)?;

    let stream_handle = OutputStreamBuilder::open_default_stream()?;
    let sink = Sink::connect_new(stream_handle.mixer());

    sink.append(source);

    sink.sleep_until_end();

    Ok(())
}
