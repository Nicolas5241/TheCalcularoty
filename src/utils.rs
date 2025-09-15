use crate::units::{HERTZ_UNITS, FARAD_UNITS, HENRY_UNITS, UnitType};

use phf::OrderedMap;

use slint::{ModelRc, SharedString, ToSharedString, VecModel};

#[inline]
pub fn vec_to_model(vec: Vec<SharedString>) -> ModelRc<SharedString> {
	ModelRc::new(VecModel::from(vec))
}

pub fn get_unit_group(value: &str) -> UnitType {
	if HERTZ_UNITS.contains_key(value) {
		return UnitType::Hertz;
	} else if FARAD_UNITS.contains_key(value) {
		return UnitType::Farad;
	} else if HENRY_UNITS.contains_key(value) {
	   return UnitType::Henry; 
	}
	UnitType::NotSelected
}

#[inline]
pub fn set_to_sharedstring_vec(set: &OrderedMap<&str, u64>) -> Vec<SharedString> {
	set.keys().into_iter().map(|x| x.to_shared_string()).collect()
}

pub fn set_new_model(new_type: &UnitType, function: impl Fn(ModelRc<SharedString>), models: &[ModelRc<SharedString>]) {
	match new_type {
		UnitType::Hertz => function(models[0].to_owned()),
		UnitType::Farad => function(models[1].to_owned()),
		UnitType::Henry => function(models[2].to_owned()),
		_ => (),
	}
}

#[inline]
pub fn get_unit_map(unit_type: &UnitType) -> &OrderedMap<&str, u64> {
    match unit_type {
        UnitType::Hertz => &HERTZ_UNITS,
        UnitType::Farad => &FARAD_UNITS,
        UnitType::Henry => &HENRY_UNITS,
        _ => unimplemented!()
    }
}

//INFO: 1/(2pi*sqrt(l*c))
fn lc_to_f0() {

}

//INFO: 1/(c*(2pi*R)²)
fn cf0_to_l() {

}

//INFO: 1/(l*(2pi*R)²)
fn lf0_to_c() {

}
