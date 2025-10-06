use crate::units::{HERTZ_UNITS, FARAD_UNITS, HENRY_UNITS};
use crate::types::UnitType;

use slint::{ModelRc, SharedString, VecModel};

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
