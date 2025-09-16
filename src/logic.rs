use crate::units::{*};
use crate::utils::{*};

use std::{cell::RefCell, error::Error, rc::Rc};

use astro_float::Consts;
use slint::{SharedString, ToSharedString};

slint::include_modules!();

pub fn start_ui() -> Result<(), Box<dyn Error>> {
	let ui = MainWindow::new()?;

	let input1_type = Rc::new(RefCell::new(UnitType::NotSelected));
	let input2_type = Rc::new(RefCell::new(UnitType::NotSelected));

	let hertz_units_shared: Vec<SharedString> = set_to_sharedstring_vec(&HERTZ_UNITS);
	let farad_units_shared: Vec<SharedString> = set_to_sharedstring_vec(&FARAD_UNITS);
	let henry_units_shared: Vec<SharedString> = set_to_sharedstring_vec(&HENRY_UNITS);

	let full_model = vec_to_model([hertz_units_shared.clone(), farad_units_shared.clone(), henry_units_shared.clone()].concat());
	
	//let hz_f_model = vec_to_model([hertz_units_shared.clone(), farad_units_shared.clone()].concat());
	//let hz_h_model = vec_to_model([hertz_units_shared, henry_units_shared.clone()].concat());
	//let f_h_model = vec_to_model([farad_units_shared, henry_units_shared].concat());

	//ui.set_lc_input1_model(full_model.clone());
	//ui.set_lc_input2_model(full_model.clone());
	//ui.set_lc_input3_model(full_model.clone());

	ui.set_lc_model(full_model);

	ui.on_lc_input1_combo_changed({
		let ui_handle = ui.as_weak();
		//let models = models_rc.clone();
		move |new_value: SharedString| {
			let ui = ui_handle.unwrap();
			handle_combobox_changed(new_value, input1_type.clone(), |value| ui.set_lc_input2_combo_text(value));
		}
	});

    ui.on_lc_input2_combo_changed({
        let ui_handle = ui.as_weak();
        move |new_value: SharedString| {
            let ui = ui_handle.unwrap();
			handle_combobox_changed(new_value, input2_type.clone(), |value| ui.set_lc_input1_combo_text(value));
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

			let input1_bigfloat = shared_to_bigfloat(input1_text);
			let input2_bigfloat = shared_to_bigfloat(input2_text);

			if input1_group == output_group {
				ui.set_lc_result_text(convert_measure(input1_bigfloat, &input1_group, input1_type, output_type).to_shared_string());
				return
			} else if input2_group == output_group {
			    ui.set_lc_result_text(convert_measure(input2_bigfloat, &input2_group, input2_type, output_type).to_shared_string());
				return
			}

			let input1_base = convert_to_base(input1_bigfloat, &input1_group, input1_type);
			let input2_base = convert_to_base(input2_bigfloat, &input2_group, input2_type);

			let result = calculate_lc(input1_base, input2_base, input1_group, output_group, &mut Consts::new().expect("idk man"));
			ui.set_lc_result_text(format_bigfloat(result).to_shared_string());
		}
	});

	ui.run()?;

	Ok(())
}

fn handle_combobox_changed(new_value: SharedString, unit_type: Rc<RefCell<UnitType>>, combo_func: impl Fn(SharedString)) {
	let new_type = get_unit_group(&new_value);

	if *unit_type.borrow() == new_type {
		return
	}
	
	combo_func("".to_shared_string());

	unit_type.replace(new_type);
}
