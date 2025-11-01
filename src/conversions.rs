use phf::OrderedMap;
use slint::SharedString;
use crate::consts::TWO_PI;
use crate::types::{BFloat, UnitType};
use crate::units::{HERTZ_UNITS, FARAD_UNITS, HENRY_UNITS, OHM_UNITS};

pub fn convert_measure(unit: BFloat, unit_type: &UnitType, unit_label: &SharedString, target_unit: &SharedString) -> BFloat {
	match unit_type {
		UnitType::Hertz => unit_convert(unit, unit_label, target_unit, &HERTZ_UNITS),
		UnitType::Farad => unit_convert(unit, unit_label, target_unit, &FARAD_UNITS),
		UnitType::Henry => unit_convert(unit, unit_label, target_unit, &HENRY_UNITS),
		UnitType::Ohm => unit_convert(unit, unit_label, target_unit, &OHM_UNITS),
	}
}

fn unit_convert(unit: BFloat, unit_label: &SharedString, target_unit: &SharedString, map: &OrderedMap<&str, i32>) -> BFloat {
	let target_ratio = *map.get(&target_unit).unwrap();
	let input_ratio = *map.get(&unit_label).unwrap();
	let ratio = input_ratio - target_ratio;

	if ratio < 0 {
		return unit / 10u32.pow(-ratio as u32).into();
	}
	unit * 10i32.pow(ratio as u32).into()
}

#[inline]
pub fn get_omega(f: BFloat) -> BFloat {
	f * TWO_PI.clone()
}
