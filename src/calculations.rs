use crate::consts::*;
use crate::types::{BFloat, UnitType};
use num_traits::{Pow, One};
use astro_float::Consts;

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

//INFO: 1/(2pi*sqrt(l*c))
pub fn lc_to_f0(l: BFloat, c: BFloat, consts: &mut Consts) -> BFloat {
	BFloat::one() / ( two_pi(consts) * (l * c).sqrt() )
}

//INFO: 1/(c*(2pi*R)²)
pub fn cf0_to_l(c: BFloat, f0: BFloat, consts_cache: &mut Consts) -> BFloat {
	BFloat::one() / ( c * ( two_pi(consts_cache) * f0 ).pow(2u8) )
}

//INFO: 1/(l*(2pi*R)²)
pub fn lf0_to_c(l: BFloat, f0: BFloat, consts_cache: &mut Consts) -> BFloat {
	BFloat::one() / ( l * ( two_pi(consts_cache) * f0 ).pow(2u8) )
}

#[inline]
fn two_pi(consts_cache: &mut Consts) -> BFloat {
	BFloat(consts_cache.pi(PRECISION, ROUNDING_MODE))
}
