use std::error::Error;

slint::include_modules!();

pub fn start_ui() -> Result<(), Box<dyn Error>> {
    let ui = MainWindow::new()?;

    ui.run()?;

    Ok(())
}
