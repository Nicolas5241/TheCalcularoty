fn main() -> Result<(), Box<dyn std::error::Error>> {
	let style: String = "native".into();
	let config = slint_build::CompilerConfiguration::new().with_style(style);

	slint_build::compile_with_config("ui/App.slint", config)?;

    Ok(())
}
