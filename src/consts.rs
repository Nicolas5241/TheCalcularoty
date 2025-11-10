use astro_float::{Consts, RoundingMode};

use crate::{types::BFloat};
use spin::{Lazy, Mutex};

pub const PRECISION: usize = 2048;
pub const ROUNDING_MODE: RoundingMode = RoundingMode::ToEven;

pub static CONSTS_CACHE: Lazy<Mutex<Consts>> = Lazy::new(|| {
	Mutex::new(Consts::new().expect("error creating const cache"))
});

pub static TWO_PI: Lazy<BFloat> = Lazy::new(||{
	BFloat::from(CONSTS_CACHE.lock().pi(PRECISION, ROUNDING_MODE)) * BFloat::from(2)
});


pub const HERTZ_BASE_TYPE: &str = "Hz";
pub const FARAD_BASE_TYPE: &str = "F";
pub const HENRY_BASE_TYPE: &str = "H";
pub const OHM_BASE_TYPE: &str = "Ω";
