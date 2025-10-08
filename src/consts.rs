use std::sync::LazyLock;

use astro_float::{Consts, RoundingMode};

use crate::{types::BFloat};

pub const PRECISION: usize = 2048;
pub const ROUNDING_MODE: RoundingMode = RoundingMode::ToEven;
pub static TWO_PI: LazyLock<BFloat> = LazyLock::new(||{
	let mut consts_cache = Consts::new().expect("consts cache could not init");
	BFloat::from(consts_cache.pi(PRECISION, ROUNDING_MODE)) * BFloat::from(2)
});
