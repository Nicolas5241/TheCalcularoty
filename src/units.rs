use phf::{phf_ordered_map, OrderedMap};
use slint::{SharedString, ToSharedString};
use crate::traits::MapToSharedStringVec;

pub static HERTZ_UNITS: OrderedMap<&str, i32> = phf_ordered_map! {
	"Hz" => 0,
    "kHz" => 3,
    "MHz" => 6,
    "GHz" => 9
};
pub static FARAD_UNITS: OrderedMap<&str, i32> = phf_ordered_map! {
	"F" => 0,
    "mF" => -3,
    "μF" => -6,
    "nF" => -9,
    "pF" => -12
};
pub static HENRY_UNITS: OrderedMap<&str, i32> = phf_ordered_map! {
	"H" => 0,
    "mH" => -3,
    "μH" => -6,
    "nH" => -9,
    "pH" => -12
};
pub static OHM_UNITS: OrderedMap<&str, i32> = phf_ordered_map! {
	"Ω" => 0,
	"kΩ" => 3,
	"MΩ" => 6,
};

impl<T> MapToSharedStringVec for OrderedMap<&str, T> {
	fn to_shared_string_vec(&self) -> Vec<SharedString> {
	    self.keys()
			.into_iter()
			.map(|x| x.to_shared_string())
			.collect::<Vec<SharedString>>()
	}
}
