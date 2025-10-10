fn main() -> Result<(), Box<dyn std::error::Error>> {
	let style: String = match std::env::consts::OS {
		"linux" => "material",
		_ => "native"
	}.to_string();

	let config = slint_build::CompilerConfiguration::new().with_style(style);

	slint_build::compile_with_config("ui/App.slint", config)?;

    Ok(())
}
