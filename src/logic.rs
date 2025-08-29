use std::{
    ffi::CString,
    io::{Cursor, Read},
    error::Error,
};
use rodio::{Decoder, OutputStreamBuilder, Sink};

#[cfg(target_os = "android")]
use ndk::asset::AssetManager;

slint::include_modules!();

#[cfg(target_os = "android")]
pub fn run_app_android(asset_manager: AssetManager) -> Result<(), Box<dyn Error>> {
    let _ = play_background_music_android(asset_manager);

    start_ui()
}

#[cfg(not(target_os = "android"))]
pub fn run_app() -> Result<(), Box<dyn Error>> {
    let _ = play_background_music();

    start_ui()
}


fn start_ui() -> Result<(), Box<dyn Error>> {

    Ok(())
}

#[cfg(target_os = "android")]
fn play_background_music_android(asset_manager: AssetManager) -> Result<(), Box<dyn Error>> {
    let music_bytes = load_asset_bytes("music.mp3", asset_manager);
    let music_cursor = Cursor::new(music_bytes);
    let source = Decoder::try_from(music_cursor)?;

    let stream_handle = OutputStreamBuilder::open_default_stream()?;
    let sink = Sink::connect_new(stream_handle.mixer());

    sink.append(source);

    sink.sleep_until_end();

    Ok(())
}

#[cfg(not(target_os = "android"))]
fn play_background_music() -> Result<(), Box<dyn Error>> {
    use rodio::OutputStream;

    let file = std::fs::File::open("assets/music.mp3")?;
    let source = Decoder::try_from(file)?;

    let stream_handle = OutputStreamBuilder::open_default_stream()?;
    let sink = Sink::connect_new(stream_handle.mixer());

    sink.append(source);
    sink.play();

    Ok(())
}

#[cfg(target_os = "android")]
fn load_asset_bytes(name: &str, mgr: AssetManager) -> Vec<u8> {
    let mut asset = mgr.open(&CString::new(name).unwrap()).expect("asset dont exist"); // relative path inside assets/
    let mut buf: Vec<u8> = Vec::new();
    let _ = asset.read_to_end(&mut buf);
    buf
}
