use std::str::FromStr;

use crate::units::{HERTZ_UNITS, FARAD_UNITS, HENRY_UNITS, UnitType};

use phf::OrderedMap;

use astro_float::{BigFloat, Consts, RoundingMode};

use regex::Regex;

use slint::{ModelRc, SharedString, ToSharedString, VecModel};

const ROUNDING_MODE: RoundingMode = RoundingMode::ToEven;
const PRECISION: usize = 2048;

pub fn format_bigfloat(num: BigFloat) -> String {
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

pub fn calculate_lc(base_input1: BigFloat, base_input2: BigFloat, base1_type: UnitType, output_type: UnitType, consts: &mut Consts) -> BigFloat {
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

pub fn convert_measure(unit: BigFloat, unit_type: &UnitType, unit_label: SharedString, target_unit: SharedString) -> BigFloat {
	match unit_type {
		UnitType::Hertz => hertz_convert(unit, unit_label, target_unit),
		UnitType::Farad => farad_convert(unit, unit_label, target_unit),
		UnitType::Henry => henry_convert(unit, unit_label, target_unit),
		UnitType::NotSelected => unimplemented!(),
	}
}

pub fn convert_to_base(unit: BigFloat, unit_type: &UnitType, unit_label: SharedString) -> BigFloat {
	match unit_type {
		UnitType::Hertz => hertz_convert(unit, unit_label, "Hz".to_shared_string()),
		UnitType::Farad => farad_convert(unit, unit_label, "F".to_shared_string()),
		UnitType::Henry => henry_convert(unit, unit_label, "H".to_shared_string()),
		UnitType::NotSelected => unimplemented!(),
	}
}

fn hertz_convert(hertz: BigFloat, unit_label: SharedString, target_unit: SharedString) -> BigFloat {
	let target_ratio = *HERTZ_UNITS.get(&target_unit).unwrap();
	let input_ratio = *HERTZ_UNITS.get(&unit_label).unwrap();
	let ratio = u64::max(target_ratio, input_ratio) / u64::min(target_ratio, input_ratio);

	if target_ratio >= input_ratio {
		return div(hertz, from_u64(ratio));
	}
	mul(hertz, from_u64(ratio))
}

fn farad_convert(farad: BigFloat, unit_label: SharedString, target_unit: SharedString) -> BigFloat {
	let target_ratio = *FARAD_UNITS.get(&target_unit).unwrap();
	let input_ratio = *FARAD_UNITS.get(&unit_label).unwrap();
	let ratio = u64::max(target_ratio, input_ratio) / u64::min(target_ratio, input_ratio);

	if target_ratio >= input_ratio {
		return mul(farad, from_u64(ratio));
	}
	div(farad, from_u64(ratio))
}

fn henry_convert(henry: BigFloat, unit_label: SharedString, target_unit: SharedString) -> BigFloat {
	let target_ratio = *HENRY_UNITS.get(&target_unit).unwrap();
	let input_ratio = *HENRY_UNITS.get(&unit_label).unwrap();
	let ratio = u64::max(target_ratio, input_ratio) / u64::min(target_ratio, input_ratio);

	if target_ratio >= input_ratio {
		return mul(henry, from_u64(ratio));
	}
	div(henry, from_u64(ratio))}

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

/*
pub fn set_new_model(new_type: &UnitType, function: impl Fn(ModelRc<SharedString>), models: &[ModelRc<SharedString>]) {
	match new_type {
		UnitType::Hertz => function(models[0].to_owned()),
		UnitType::Farad => function(models[1].to_owned()),
		UnitType::Henry => function(models[2].to_owned()),
		_ => (),
	}
}
*/

/*
#[inline]
pub fn get_unit_map(unit_type: &UnitType) -> &OrderedMap<&str, u64> {
    match unit_type {
        UnitType::Hertz => &HERTZ_UNITS,
        UnitType::Farad => &FARAD_UNITS,
        UnitType::Henry => &HENRY_UNITS,
        _ => unimplemented!()
    }
}
*/

#[inline]
pub fn shared_to_bigfloat(str: SharedString) -> BigFloat {
	BigFloat::from_str(&str).unwrap()
}

//INFO: 1/(2pi*sqrt(l*c))
fn lc_to_f0(l: BigFloat, c: BigFloat, consts: &mut Consts) -> BigFloat {
	mul(
		mul(
			BigFloat::from_u64(2, PRECISION),
			pi(consts)
		),
		sqrt(
			mul(
				l,
				c
			)
		)
	).reciprocal(PRECISION, ROUNDING_MODE)
}

//INFO: 1/(c*(2pi*R)²)
fn cf0_to_l(c: BigFloat, f0: BigFloat, consts_cache: &mut Consts) -> BigFloat {
	mul(
		c,
		pow(
			mul(
				mul(
					BigFloat::from_u64(2, PRECISION),
					pi(consts_cache)
				),
				f0
			),
			2
		)
	).reciprocal(PRECISION, ROUNDING_MODE)
}

//INFO: 1/(l*(2pi*R)²)
fn lf0_to_c(l: BigFloat, f0: BigFloat, consts_cache: &mut Consts) -> BigFloat {
	mul(l,
		pow(
			mul(f0,
				mul(
					BigFloat::from_u64(2, PRECISION),
					pi(consts_cache)
				)
			),
			2
		)
	).reciprocal(PRECISION, ROUNDING_MODE)
}

#[inline]
fn from_u64(num: u64) -> BigFloat {
	BigFloat::from_u64(num, PRECISION)
}

#[inline]
fn mul(n1: BigFloat, n2: BigFloat) -> BigFloat {
	n1.mul_full_prec(&n2)
}

#[inline]
fn div(n1: BigFloat, n2: BigFloat) -> BigFloat {
	n1.div(&n2, PRECISION, ROUNDING_MODE)
}

#[inline]
fn sqrt(n: BigFloat) -> BigFloat {
	n.sqrt(PRECISION, ROUNDING_MODE)
}

#[inline]
fn pow(n: BigFloat, p: usize) -> BigFloat {
	n.powi(p, PRECISION, ROUNDING_MODE)
}

#[inline]
fn pi(consts_cache: &mut Consts) -> BigFloat {
	consts_cache.pi(PRECISION, ROUNDING_MODE)
}
