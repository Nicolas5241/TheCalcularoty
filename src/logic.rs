use crate::units::{*};
use crate::utils::{*};

use std::{cell::RefCell, error::Error, rc::Rc};

use slint::{ModelRc, SharedString, ToSharedString};

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

	let models_rc = Rc::new([f_h_model, hz_h_model, hz_f_model]);

	ui.set_lc_input1_model(full_model.clone());
	ui.set_lc_input2_model(full_model.clone());
	ui.set_lc_input3_model(full_model.clone());

	ui.on_lc_input1_combo_changed({
		let ui_handle = ui.as_weak();
		let models = models_rc.clone();
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
				models.as_ref()
			);

			input1_type.replace(new_type);
		}
	});

    ui.on_lc_input2_combo_changed({
        let ui_handle = ui.as_weak();
		let models = models_rc.clone();
        move |new_value: SharedString| {
            let ui = ui_handle.unwrap();
			let new_type = get_unit_group(&new_value);

			if *input2_type.borrow() == new_type {
				return
			}
			set_new_model(
				&new_type,
				|value: ModelRc<SharedString>| {
					ui.set_lc_input1_model(value);
				},
				models.as_ref()
			);

			input2_type.replace(new_type);
        }
    });

	ui.on_lc_calcularot({
		let ui_handle = ui.as_weak();
		move |input1_type, input2_type, output_type, input1_text, input2_text| {
			let ui = ui_handle.unwrap();

			let input1_group = get_unit_group(&input1_type);
			let input2_group = get_unit_group(&input2_type);
			let output_group = get_unit_group(&output_type);

			println!("input1 grp {:?}\ninput2 grp {:?}\ninput3 grp {:?}", input1_group, input2_group, output_group);

			if input1_group == UnitType::NotSelected || input2_group == UnitType::NotSelected || output_group == UnitType::NotSelected {
				return
			}

			if input1_group == output_group {
				ui.set_lc_result_text(convert_measure(shared_to_bigfloat(input1_text), input1_group, input1_type, output_type).to_shared_string());
			} else if input2_group == output_group {
			    ui.set_lc_result_text(convert_measure(shared_to_bigfloat(input2_text), input2_group, input2_type, output_type).to_shared_string());
			}

		}
	});

	ui.run()?;

	Ok(())
}
