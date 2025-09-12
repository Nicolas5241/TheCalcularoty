use std::error::Error;

use slint::{Model, ModelRc, SharedString, ToSharedString, VecModel};

slint::include_modules!();

enum UnitType {
    Hertz,
    Farad,
    Henry,
}

pub fn start_ui() -> Result<(), Box<dyn Error>> {
    let ui = MainWindow::new()?;

	let hertz_units: &[SharedString] = &["Hz".into(), "kHz".into(), "mHz".into(), "gHz".into()];
	let farad_units: &[SharedString] = &["F".into(), "mF".into(), "μF".into(), "nF".into(), "pF".into()];
	let henry_units: &[SharedString] = &["H".into(), "mH".into(), "μH".into(), "nH".into(), "pH".into()];

	let full_model = vec_to_model([hertz_units, farad_units, henry_units].concat());
	
    let hz_f_model = vec_to_model([hertz_units, farad_units].concat());
    let hz_h_model = vec_to_model([hertz_units, henry_units].concat());
    let f_h_model = vec_to_model([farad_units, henry_units].concat());
	let hertz_model = VecModel::from_slice(hertz_units);
	let farad_model = VecModel::from_slice(farad_units);
	let henry_model = VecModel::from_slice(henry_units);

	ui.set_lc_input1_model(full_model.clone());
	ui.set_lc_input2_model(full_model.clone());
	ui.set_lc_input3_model(full_model.clone());


    ui.run()?;

    Ok(())
}

#[inline]
fn vec_to_model(vec: Vec<SharedString>) -> ModelRc<SharedString> {
	ModelRc::new(VecModel::from(vec))
}

fn get_unit_group(value: SharedString) -> UnitType {
    todo!()
}
