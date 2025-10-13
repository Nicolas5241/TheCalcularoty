use crate::types::*;
use crate::traits::MapToSharedStringVec;

use crate::units::{*};
use crate::utils::{*};
use crate::calculations::*;
use crate::conversions::*;

use std::str::FromStr;
use std::{cell::RefCell, error::Error, rc::Rc};

use slint::{SharedString, ToSharedString};

slint::include_modules!();

pub fn start_ui() -> Result<(), Box<dyn Error>> {
	let ui = MainWindow::new()?;

	let input1_type = Rc::new(RefCell::new(UnitType::NotSelected));
	let input2_type = Rc::new(RefCell::new(UnitType::NotSelected));

	let hertz_units_shared: Vec<SharedString> = HERTZ_UNITS.to_shared_string_vec();
	let farad_units_shared: Vec<SharedString> = FARAD_UNITS.to_shared_string_vec();
	let henry_units_shared: Vec<SharedString> = HENRY_UNITS.to_shared_string_vec();

	let full_model = vec_to_model([hertz_units_shared.clone(), farad_units_shared.clone(), henry_units_shared.clone()].concat());
	
	//let hz_f_model = vec_to_model([hertz_units_shared.clone(), farad_units_shared.clone()].concat());
	//let hz_h_model = vec_to_model([hertz_units_shared, henry_units_shared.clone()].concat());
	//let f_h_model = vec_to_model([farad_units_shared, henry_units_shared].concat());

	//ui.set_lc_input1_model(full_model.clone());
	//ui.set_lc_input2_model(full_model.clone());
	//ui.set_lc_input3_model(full_model.clone());

	ui.set_lc_model(full_model);

	ui.set_l_model(vec_to_model(henry_units_shared));
	ui.set_c_model(vec_to_model(farad_units_shared));
	ui.set_f_model(vec_to_model(hertz_units_shared));

	ui.on_lc_input1_combo_changed({
		let ui_handle = ui.as_weak();
		let type1 = input1_type.clone();
		let type2 = input2_type.clone();
		//let models = models_rc.clone();
		move |new_value: SharedString| {
			let ui = ui_handle.unwrap();
			handle_combobox_changed(new_value, type1.clone(), type2.clone(), |value| ui.set_lc_input2_combo_text(value));
		}
	});

    ui.on_lc_input2_combo_changed({
        let ui_handle = ui.as_weak();
		//let type1 = input2_type.clone();
		//let type2 = input1_type.clone();
        move |new_value: SharedString| {
            let ui = ui_handle.unwrap();
			handle_combobox_changed(new_value, input2_type.clone(), input1_type.clone(), |value| ui.set_lc_input1_combo_text(value));
        }
    });

	ui.on_lc_calcularot({
		let ui_handle = ui.as_weak();
		move |input1_type, input2_type, output_type, input1_text, input2_text| {
			let ui = ui_handle.unwrap();

			let input1_group = get_unit_group(&input1_type);
			let input2_group = get_unit_group(&input2_type);
			let output_group = get_unit_group(&output_type);

			if input1_group == UnitType::NotSelected || input2_group == UnitType::NotSelected || output_group == UnitType::NotSelected {
				return
			}

			let input1_bigfloat: BFloat = BFloat::from_str(&input1_text).unwrap();
			let input2_bigfloat: BFloat = BFloat::from_str(&input2_text).unwrap();

			if input1_group == output_group {
				ui.set_lc_result_text(
					convert_measure(input1_bigfloat, &input1_group, input1_type, output_type).to_shared_string()
				);
				return
			} else if input2_group == output_group {
			    ui.set_lc_result_text(
					convert_measure(input2_bigfloat, &input2_group, input2_type, output_type).to_shared_string()
				);
				return
			}

			let input1_base = convert_to_base(input1_bigfloat, &input1_group, input1_type);
			let input2_base = convert_to_base(input2_bigfloat, &input2_group, input2_type);

			let result = calculate_lc(input1_base, input2_base, input1_group, output_group);
			ui.set_lc_result_text(result.to_shared_string());
		}
	});

	ui.on_imp_calcularot({
		let ui_handle = ui.as_weak();
		move |l_str, c_str, f_str, type_index| {
			let ui = ui_handle.unwrap();
		}
	});

	ui.run()?;

	Ok(())
}

fn handle_combobox_changed(new_value: SharedString, unit_type: Rc<RefCell<UnitType>>, other_type: Rc<RefCell<UnitType>>, combo_func: impl Fn(SharedString)) {
	let new_type = get_unit_group(&new_value);
	
	if *unit_type.borrow() == new_type {
		return
	}

	if new_type == *other_type.borrow() {
		combo_func("".to_shared_string());
	}

	unit_type.replace(new_type);
}
