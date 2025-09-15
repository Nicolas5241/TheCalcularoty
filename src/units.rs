use phf::{phf_ordered_map, OrderedMap};

#[derive(PartialEq, Eq)]
pub enum UnitType {
	Hertz,
	Farad,
	Henry,
	NotSelected,
}

pub static HERTZ_UNITS: OrderedMap<&str, u64> = phf_ordered_map! {
	"Hz" => 1,
    "kHz" => 1000,
    "mHz" => 1000000,
    "gHz" => 1000000000
};
pub static FARAD_UNITS: OrderedMap<&str, u64> = phf_ordered_map! {
	"F" => 1,
    "mF" => 1000,
    "μF" => 1000000,
    "nF" => 1000000000,
    "pF" => 1000000000000
};
pub static HENRY_UNITS: OrderedMap<&str, u64> = phf_ordered_map! {
	"H" => 1,
    "mH" => 1000,
    "μH" => 1000000,
    "nH" => 1000000000,
    "pH" => 1000000000000
};
