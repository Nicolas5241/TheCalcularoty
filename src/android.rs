use std::{
    ffi::CString,
    io::{Cursor, Read},
};
use rodio::{Decoder, OutputStreamBuilder, Sink};
use ndk::asset::AssetManager;

#[path = "logic.rs"]
mod logic;

pub fn run_app(asset_manager: AssetManager) -> Result<(), Box<dyn Error>> {
    std::thread::spawn(||{
        let _ = play_background_music_android(asset_manager);
    });

    logic::start_ui()
}

fn play_background_music_android(asset_manager: AssetManager) -> Result<(), Box<dyn Error>> {
    let music_bytes = load_asset_bytes("music.mp3", asset_manager);
    let music_cursor = Cursor::new(music_bytes);
    let source = Decoder::new_looped(music_cursor)?;

    let stream_handle = OutputStreamBuilder::open_default_stream()?;
    let sink = Sink::connect_new(stream_handle.mixer());

    sink.append(source);

    sink.sleep_until_end();

    Ok(())
}

fn load_asset_bytes(name: &str, mgr: AssetManager) -> Vec<u8> {
    let mut asset = mgr.open(&CString::new(name).unwrap()).expect("asset dont exist");
    let mut buf: Vec<u8> = Vec::new();
    let _ = asset.read_to_end(&mut buf);
    buf
}
