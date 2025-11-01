use crate::consts::*;
use crate::types::{BFloat};
use num_complex::Complex;
use num_traits::{One, Pow, Zero};

//INFO: 1/(2pi*sqrt(l*c))
pub fn lc_to_f0(l: BFloat, c: BFloat) -> BFloat {
	BFloat::one() / ( TWO_PI.clone() * (l * c).sqrt() )
}

//INFO: 1/(c*(2pi*R)²)
pub fn cf0_to_l(c: BFloat, f0: BFloat) -> BFloat {
	BFloat::one() / ( c * ( TWO_PI.clone() * f0 ).pow(2u8) )
}

//INFO: 1/(l*(2pi*R)²)
pub fn lf0_to_c(l: BFloat, f0: BFloat) -> BFloat {
	BFloat::one() / ( l * ( TWO_PI.clone() * f0 ).pow(2u8) )
}

pub fn lc_inductive_reactance(l: BFloat, omega: BFloat) -> Complex<BFloat> {

	Complex::new(BFloat::zero(), omega * l)
}

pub fn lc_capacitive_reactance(c: BFloat, omega: BFloat) -> Complex<BFloat> {
	Complex::new(BFloat::zero(), -BFloat::one() / (omega * c))
}

pub fn calculate_impedance_series(l: BFloat, c: BFloat, omega: BFloat) -> (BFloat, BFloat, BFloat) {
	let l_reactance = lc_inductive_reactance(l, omega.clone()).im;
	let c_reactance = lc_capacitive_reactance(c, omega).im;

	let lc_impedance = l_reactance.clone() + c_reactance.clone();

	(lc_impedance.abs(), l_reactance, c_reactance.abs())
}

pub fn calculate_impedance_parallel(l: BFloat, c: BFloat, omega: BFloat) -> (BFloat, BFloat, BFloat) {
	let l_reactance = lc_inductive_reactance(l, omega.clone()).im;
	let c_reactance = lc_capacitive_reactance(c, omega).im;

	let lc_impedance = (l_reactance.clone() * c_reactance.clone()) / (l_reactance.clone() + c_reactance.clone());

	(lc_impedance.abs(), l_reactance, c_reactance.abs())
}

#[inline]
pub fn calculate_resonant_frequency(l: BFloat, c: BFloat) -> BFloat {
	BFloat::one() / ( TWO_PI.clone() * ( l * c ).sqrt() )
}
