use crate::units::{HERTZ_UNITS, FARAD_UNITS, HENRY_UNITS};
use crate::types::*;

use num_traits::{One, Pow};
use phf::OrderedMap;

use astro_float::{Consts, RoundingMode};

use regex::Regex;

use slint::{ModelRc, SharedString, ToSharedString, VecModel};

const ROUNDING_MODE: RoundingMode = RoundingMode::ToEven;
const PRECISION: usize = 2048;

pub fn format_bfloat(num: BFloat) -> String {
	let reg = Regex::new(r"^(.*?)e(.*?)$").unwrap();
	let res_string = num.to_string();
	let captures = reg.captures(&res_string).unwrap();
	let num_str = captures.get(1).unwrap().as_str();
	let exp_str = captures.get(2).unwrap().as_str();
	let mut num = num_str.parse::<f64>().unwrap();
	let mut exp = exp_str.parse::<i32>().unwrap();
	while num >= 10. {
		num /= 10.;
		exp += 1;
	}
	num = (num * 1e14).round()/1e14;
	if (-14..=14).contains(&exp) {
		return format!("~{}", num * 10f64.powi(exp));
	}
	format!("~{num}e{exp}")
}

#[inline]
pub fn vec_to_model(vec: Vec<SharedString>) -> ModelRc<SharedString> {
	ModelRc::new(VecModel::from(vec))
}

pub fn calculate_lc(base_input1: BFloat, base_input2: BFloat, base1_type: UnitType, output_type: UnitType, consts: &mut Consts) -> BFloat {
	match output_type {
		UnitType::Hertz => {
			if base1_type == UnitType::Henry {
				return lc_to_f0(base_input1, base_input2, consts);
			}
			lc_to_f0(base_input2, base_input1, consts)
		}

		UnitType::Farad => {
			if base1_type == UnitType::Henry {
				return lf0_to_c(base_input1, base_input2, consts);
			}
			lf0_to_c(base_input2, base_input1, consts)
		}

		UnitType::Henry => {
			if base1_type == UnitType::Farad {
				return cf0_to_l(base_input1, base_input2, consts);
			}
			cf0_to_l(base_input1, base_input2, consts)
		}

		UnitType::NotSelected => unimplemented!()
	}
}

pub fn convert_measure(unit: BFloat, unit_type: &UnitType, unit_label: SharedString, target_unit: SharedString) -> BFloat {
	match unit_type {
		UnitType::Hertz => hertz_convert(unit, unit_label, target_unit),
		UnitType::Farad => farad_convert(unit, unit_label, target_unit),
		UnitType::Henry => henry_convert(unit, unit_label, target_unit),
		UnitType::NotSelected => unimplemented!(),
	}
}

pub fn convert_to_base(unit: BFloat, unit_type: &UnitType, unit_label: SharedString) -> BFloat {
	match unit_type {
		UnitType::Hertz => hertz_convert(unit, unit_label, "Hz".to_shared_string()),
		UnitType::Farad => farad_convert(unit, unit_label, "F".to_shared_string()),
		UnitType::Henry => henry_convert(unit, unit_label, "H".to_shared_string()),
		UnitType::NotSelected => unimplemented!(),
	}
}

fn hertz_convert(hertz: BFloat, unit_label: SharedString, target_unit: SharedString) -> BFloat {
	let target_ratio = *HERTZ_UNITS.get(&target_unit).unwrap();
	let input_ratio = *HERTZ_UNITS.get(&unit_label).unwrap();
	let ratio = u64::max(target_ratio, input_ratio) / u64::min(target_ratio, input_ratio);

	if target_ratio >= input_ratio {
		return hertz / ratio.into();
	}
	hertz * ratio.into()
}

fn farad_convert(farad: BFloat, unit_label: SharedString, target_unit: SharedString) -> BFloat {
	let target_ratio = *FARAD_UNITS.get(&target_unit).unwrap();
	let input_ratio = *FARAD_UNITS.get(&unit_label).unwrap();
	let ratio = u64::max(target_ratio, input_ratio) / u64::min(target_ratio, input_ratio);

	if target_ratio >= input_ratio {
		return farad * ratio.into();
	}
	farad / ratio.into()
}

fn henry_convert(henry: BFloat, unit_label: SharedString, target_unit: SharedString) -> BFloat {
	let target_ratio = *HENRY_UNITS.get(&target_unit).unwrap();
	let input_ratio = *HENRY_UNITS.get(&unit_label).unwrap();
	let ratio = u64::max(target_ratio, input_ratio) / u64::min(target_ratio, input_ratio);

	if target_ratio >= input_ratio {
		return henry * ratio.into();
	}
	henry / ratio.into()
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

//INFO: 1/(2pi*sqrt(l*c))
fn lc_to_f0(l: BFloat, c: BFloat, consts: &mut Consts) -> BFloat {
	BFloat::one() / ( two_pi(consts) * (l * c).sqrt() )
}

//INFO: 1/(c*(2pi*R)²)
fn cf0_to_l(c: BFloat, f0: BFloat, consts_cache: &mut Consts) -> BFloat {
	BFloat::one() / ( c * ( two_pi(consts_cache) * f0 ).pow(2u8) )
}

//INFO: 1/(l*(2pi*R)²)
fn lf0_to_c(l: BFloat, f0: BFloat, consts_cache: &mut Consts) -> BFloat {
	BFloat::one() / ( l * ( two_pi(consts_cache) * f0 ).pow(2u8) )
}

#[inline]
fn two_pi(consts_cache: &mut Consts) -> BFloat {
	BFloat(consts_cache.pi(PRECISION, ROUNDING_MODE))
}
