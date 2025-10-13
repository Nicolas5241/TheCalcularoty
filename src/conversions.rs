use slint::{SharedString, ToSharedString};
use crate::consts::{TWO_PI, HENRY_BASE_TYPE, HERTZ_BASE_TYPE, FARAD_BASE_TYPE};
use crate::types::*;
use crate::units::*;

pub fn convert_measure(unit: BFloat, unit_type: &UnitType, unit_label: &SharedString, target_unit: &SharedString) -> BFloat {
	match unit_type {
		UnitType::Hertz => hertz_convert(unit, unit_label, target_unit),
		UnitType::Farad => farad_convert(unit, unit_label, target_unit),
		UnitType::Henry => henry_convert(unit, unit_label, target_unit),
		UnitType::NotSelected => unimplemented!(),
	}
}

pub fn convert_to_base(unit: BFloat, unit_type: &UnitType, unit_label: &SharedString) -> BFloat {
	match unit_type {
		UnitType::Hertz => hertz_convert(unit, unit_label, &HERTZ_BASE_TYPE.to_shared_string()),
		UnitType::Farad => farad_convert(unit, unit_label, &FARAD_BASE_TYPE.to_shared_string()),
		UnitType::Henry => henry_convert(unit, unit_label, &HENRY_BASE_TYPE.to_shared_string()),
		UnitType::NotSelected => unimplemented!(),
	}
}

fn hertz_convert(hertz: BFloat, unit_label: &SharedString, target_unit: &SharedString) -> BFloat {
	let target_ratio = *HERTZ_UNITS.get(&target_unit).unwrap();
	let input_ratio = *HERTZ_UNITS.get(&unit_label).unwrap();
	let ratio = u64::max(target_ratio, input_ratio) / u64::min(target_ratio, input_ratio);

	if target_ratio >= input_ratio {
		return hertz / ratio.into();
	}
	hertz * ratio.into()
}

fn farad_convert(farad: BFloat, unit_label: &SharedString, target_unit: &SharedString) -> BFloat {
	let target_ratio = *FARAD_UNITS.get(&target_unit).unwrap();
	let input_ratio = *FARAD_UNITS.get(&unit_label).unwrap();
	let ratio = u64::max(target_ratio, input_ratio) / u64::min(target_ratio, input_ratio);

	if target_ratio >= input_ratio {
		return farad * ratio.into();
	}
	farad / ratio.into()
}

fn henry_convert(henry: BFloat, unit_label: &SharedString, target_unit: &SharedString) -> BFloat {
	let target_ratio = *HENRY_UNITS.get(&target_unit).unwrap();
	let input_ratio = *HENRY_UNITS.get(&unit_label).unwrap();
	let ratio = u64::max(target_ratio, input_ratio) / u64::min(target_ratio, input_ratio);

	if target_ratio >= input_ratio {
		return henry * ratio.into();
	}
	henry / ratio.into()
}

pub fn hz_to_omega(hz: BFloat) -> BFloat {
	hz * TWO_PI.clone()
}
