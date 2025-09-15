use crate::units::{*};
use crate::utils::{*};

use std::{cell::RefCell, error::Error, rc::Rc};

use slint::{ModelRc, SharedString};

slint::include_modules!();

pub fn start_ui() -> Result<(), Box<dyn Error>> {
	let ui = MainWindow::new()?;

	let input1_type = Rc::new(RefCell::new(UnitType::NotSelected));
	let input2_type = Rc::new(RefCell::new(UnitType::NotSelected));

	let hertz_units_shared: Vec<SharedString> = set_to_sharedstring_vec(&HERTZ_UNITS);
	let farad_units_shared: Vec<SharedString> = set_to_sharedstring_vec(&FARAD_UNITS);
	let henry_units_shared: Vec<SharedString> = set_to_sharedstring_vec(&HENRY_UNITS);

	let full_model = vec_to_model([hertz_units_shared.clone(), farad_units_shared.clone(), henry_units_shared.clone()].concat());
	
	let hz_f_model = vec_to_model([hertz_units_shared.clone(), farad_units_shared.clone()].concat());
	let hz_h_model = vec_to_model([hertz_units_shared, henry_units_shared.clone()].concat());
	let f_h_model = vec_to_model([farad_units_shared, henry_units_shared].concat());

	let models_slice = [f_h_model, hz_h_model, hz_f_model];

	ui.set_lc_input1_model(full_model.clone());
	ui.set_lc_input2_model(full_model.clone());
	ui.set_lc_input3_model(full_model.clone());

	ui.on_lc_input1_combo_changed({
		let ui_handle = ui.as_weak();
		move |new_value: SharedString| {
			let ui = ui_handle.unwrap();
			let new_type = get_unit_group(&new_value);

			if *input1_type.borrow() == new_type {
				return
			}
	
			set_new_model(
				&new_type,
				|value: ModelRc<SharedString>| {
					ui.set_lc_input2_model(value);
				},
				&models_slice
			);

			input1_type.replace(new_type);
		}
	});

	ui.run()?;

	Ok(())
}
