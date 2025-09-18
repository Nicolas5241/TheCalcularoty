fn main() -> Result<(), Box<dyn std::error::Error>> {
	let config = slint_build::CompilerConfiguration::new().with_style("native".into());

	slint_build::compile_with_config("ui/App.slint", config)?;

    Ok(())
}
