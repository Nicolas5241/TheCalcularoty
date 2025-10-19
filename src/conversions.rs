use phf::OrderedMap;
use slint::{SharedString, ToSharedString};
use crate::consts::{FARAD_BASE_TYPE, HENRY_BASE_TYPE, HERTZ_BASE_TYPE, OHM_BASE_TYPE, TWO_PI};
use crate::types::*;
use crate::units::*;

pub fn convert_measure(unit: BFloat, unit_type: &UnitType, unit_label: &SharedString, target_unit: &SharedString) -> BFloat {
	match unit_type {
		UnitType::Hertz => unit_convert(unit, unit_label, target_unit, &HERTZ_UNITS),
		UnitType::Farad => unit_convert(unit, unit_label, target_unit, &FARAD_UNITS),
		UnitType::Henry => unit_convert(unit, unit_label, target_unit, &HENRY_UNITS),
		UnitType::Ohm => unit_convert(unit, unit_label, target_unit, &OHM_UNITS),
		UnitType::NotSelected => unimplemented!(),
	}
}

pub fn convert_to_base(unit: BFloat, unit_type: &UnitType, unit_label: &SharedString) -> BFloat {
	match unit_type {
		UnitType::Hertz => unit_convert(unit, unit_label, &HERTZ_BASE_TYPE.to_shared_string(), &HERTZ_UNITS),
		UnitType::Farad => unit_convert(unit, unit_label, &FARAD_BASE_TYPE.to_shared_string(), &FARAD_UNITS),
		UnitType::Henry => unit_convert(unit, unit_label, &HENRY_BASE_TYPE.to_shared_string(), &HENRY_UNITS),
		UnitType::Ohm => unit_convert(unit, unit_label, &OHM_BASE_TYPE.to_shared_string(), &OHM_UNITS),
		UnitType::NotSelected => unimplemented!(),
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
