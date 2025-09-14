use std::{cell::RefCell, error::Error, rc::Rc};

use phf::{phf_ordered_set, OrderedSet};

use slint::{ModelRc, SharedString, ToSharedString, VecModel};

slint::include_modules!();

#[derive(PartialEq, Eq)]
enum UnitType {
	Hertz,
	Farad,
	Henry,
	NotSelected,
}

static HERTZ_UNITS: OrderedSet<&str> = phf_ordered_set! {
	"Hz", "kHz", "mHz", "gHz"
};
static FARAD_UNITS: OrderedSet<&str> = phf_ordered_set! {
	"F", "mF", "μF", "nF", "pF"
};
static HENRY_UNITS: OrderedSet<&str> = phf_ordered_set! {
	"H", "mH", "μH", "nH", "pH"
};

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

#[inline]
fn vec_to_model(vec: Vec<SharedString>) -> ModelRc<SharedString> {
	ModelRc::new(VecModel::from(vec))
}

fn get_unit_group(value: &str) -> UnitType {
	if HERTZ_UNITS.contains(value) {
		return UnitType::Hertz;
	} else if FARAD_UNITS.contains(value) {
		return UnitType::Farad;
	} else if HENRY_UNITS.contains(value) {
	   return UnitType::Henry; 
	}
	UnitType::NotSelected
}

#[inline]
fn set_to_sharedstring_vec(set: &OrderedSet<&str>) -> Vec<SharedString> {
	set.iter().map(|x| x.to_shared_string()).collect()
}

fn set_new_model(new_type: &UnitType, function: impl Fn(ModelRc<SharedString>), models: &[ModelRc<SharedString>]) {
	match new_type {
		UnitType::Hertz => function(models[0].to_owned()),
		UnitType::Farad => function(models[1].to_owned()),
		UnitType::Henry => function(models[2].to_owned()),
		_ => (),
	}
}
