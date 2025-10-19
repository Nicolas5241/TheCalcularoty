use crate::consts::{FARAD_BASE_TYPE, HENRY_BASE_TYPE, HERTZ_BASE_TYPE, OHM_BASE_TYPE};
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
	let ohm_units_shared: Vec<SharedString> = OHM_UNITS.to_shared_string_vec();

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
	ui.set_r_model(vec_to_model(ohm_units_shared));

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
					convert_measure(input1_bigfloat, &input1_group, &input1_type, &output_type).to_shared_string()
				);
				return
			} else if input2_group == output_group {
			    ui.set_lc_result_text(
					convert_measure(input2_bigfloat, &input2_group, &input2_type, &output_type).to_shared_string()
				);
				return
			}

			let input1_base = convert_to_base(input1_bigfloat, &input1_group, &input1_type);
			let input2_base = convert_to_base(input2_bigfloat, &input2_group, &input2_type);

			let result = calculate_lc(input1_base, input2_base, input1_group, output_group);
			ui.set_lc_result_text(result.to_shared_string());
		}
	});

	ui.on_imp_calcularot({
		let ui_handle = ui.as_weak();
		move |l_str, c_str, f_str, l_type, c_type, f_type, type_index, imp_type, xl_type, xc_type, rf_type| {
			let ui = ui_handle.unwrap();
			let values_option = get_full_value_list(&l_str, &c_str, &f_str, &l_type, &c_type, &f_type, &ui);

			if values_option.is_none() {
				return;
			}

			let (l, c, f) = values_option.unwrap();

			let omega = get_omega(f);

			println!("{}", type_index);

			let (impedance, inductive_reactance, capacitive_reactance) = match type_index {
				0 => calculate_impedance_series(l.clone(), c.clone(), omega),
				1 => calculate_impedance_parallel(l.clone(), c.clone(), omega),
				_ => unreachable!()
			};

			let resonant_frequency = calculate_resonant_frequency(l, c);

			let impedance_target = convert_measure(impedance, &UnitType::Ohm, &OHM_BASE_TYPE.to_shared_string(), &imp_type);
			let xl_target = convert_measure(inductive_reactance, &UnitType::Ohm, &OHM_BASE_TYPE.to_shared_string(), &xl_type);
			let xc_target = convert_measure(capacitive_reactance, &UnitType::Ohm, &OHM_BASE_TYPE.to_shared_string(), &xc_type);
			let rf_target = convert_measure(resonant_frequency, &UnitType::Hertz, &HERTZ_BASE_TYPE.to_shared_string(), &rf_type);

			ui.set_impedance(impedance_target.as_decimal_string().into());
			ui.set_inductive_reactance(xl_target.as_decimal_string().into());
			ui.set_capacitive_reactance(xc_target.as_decimal_string().into());
			ui.set_resonant_frequency(rf_target.as_decimal_string().into());
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

fn get_full_value_list(l_str: &SharedString, c_str: &SharedString, f_str: &SharedString, l_type: &SharedString, c_type: &SharedString, f_type: &SharedString, ui: &MainWindow) -> Option<(BFloat, BFloat, BFloat)> {
	let maybe_expect_message = "astro_float changed their FromStr implementation";

	let l: BFloat;
	let c: BFloat;
	let f: BFloat;

	let l_maybe = BFloat::from_str(&l_str).expect(maybe_expect_message);
	let c_maybe = BFloat::from_str(&c_str).expect(maybe_expect_message);
	let f_maybe = BFloat::from_str(&f_str).expect(maybe_expect_message);

	let l_nan = l_maybe.0.is_nan();
	let c_nan = c_maybe.0.is_nan();
	let f_nan = f_maybe.0.is_nan();

	let nans = l_nan as u8 + c_nan as u8 + f_nan as u8;

	if nans > 1 {
		return None;
	}

	let l_maybe_base = match l_nan {
		true => None,
		false => Some(convert_to_base(l_maybe, &UnitType::Henry, l_type)),
	};

	let c_maybe_base = match c_nan {
		true => None,
		false => Some(convert_to_base(c_maybe, &UnitType::Farad, c_type)),
	};
	
	let f_maybe_base = match f_nan {
		true => None,
		false => Some(convert_to_base(f_maybe, &UnitType::Hertz, f_type)),
	};

	l = match l_maybe_base.clone() {
		Some(num) => num,
		None => {
			let value_base = cf0_to_l(c_maybe_base.clone().unwrap(), f_maybe_base.clone().unwrap());

			let value = convert_measure(value_base, &UnitType::Henry, &HENRY_BASE_TYPE.to_shared_string(), l_type);

			ui.set_inductance(value.as_decimal_string().to_shared_string());

			value
		},
	};

	c = match c_maybe_base.clone() {
		Some(num) => num,
		None => {
			let value_base = lf0_to_c(l.clone(), f_maybe_base.clone().unwrap());

			let value = convert_measure(value_base, &UnitType::Farad, &FARAD_BASE_TYPE.to_shared_string(), &c_type);

			ui.set_capacitance(value.as_decimal_string().to_shared_string());

			value
		},
	};

	f = match f_maybe_base {
		Some(num) => num,
		None => {
			let value_base = lc_to_f0(l_maybe_base.unwrap(), c_maybe_base.unwrap());

			let value = convert_measure(value_base, &UnitType::Hertz, &HERTZ_BASE_TYPE.to_shared_string(), &f_type);

			ui.set_frequency(value.as_decimal_string().to_shared_string());

			value
		},
	};

	Some((l, c, f))
}
