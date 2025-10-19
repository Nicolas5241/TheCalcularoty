use std::{fmt::Display, ops::{Add, Div, Mul, Neg, Sub}, str::FromStr};
use regex::Regex;
use astro_float::{BigFloat, RoundingMode};
use num_traits::{One, Pow, Zero};

use crate::utils;

const PRECISION: usize = 1024;
const ROUNDING_MODE: RoundingMode = RoundingMode::ToEven;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum UnitType {
	Hertz,
	Farad,
	Henry,
	Ohm,
	NotSelected,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BFloat(pub BigFloat);

macro_rules! impl_from {
    ($($t:ty),*) => {
        $(
            impl From<$t> for BFloat {
                fn from(x: $t) -> Self {
                    BigFloat::from(x).into()
                }
            }
        )*
    }
}

impl FromStr for BFloat {
	type Err = Box<dyn std::error::Error>;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let value: BFloat = BigFloat::from_str(s)?.into();

		Ok(value)
	}
}

impl Add for BFloat {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		self.0.add_full_prec(&rhs.0).into()
	}
}

impl Sub for BFloat {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
	    self.0.sub_full_prec(&rhs.0).into()
	}
}

impl Mul for BFloat {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output {
	    self.0.mul_full_prec(&rhs.0).into()
	}
}

impl Div for BFloat {
	type Output = Self;
	fn div(self, rhs: Self) -> Self::Output {
	    self.0.div(&rhs.0, PRECISION, ROUNDING_MODE).into()
	}
}

impl<T> Pow<T> for BFloat
	where T: Into<usize>,
{
	type Output = Self;
	fn pow(self, rhs: T) -> Self::Output {
		self.0.powi(rhs.into(), PRECISION, ROUNDING_MODE).into()
	}
}

impl BFloat {
	pub fn sqrt(&self) -> Self {
		self.0.sqrt(PRECISION, ROUNDING_MODE).into()
	}

	pub fn abs(&self) -> Self {
		BFloat(self.0.abs())
	}
}

impl Display for BFloat {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let reg = Regex::new(r"^(.*?)e(.*?)$").unwrap();
		let res_string = self.0.to_string();
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
			return write!(f, "~{}", num * 10f64.powi(exp));
		}

		write!(f, "{num}e{exp}")
	}
}

impl Neg for BFloat {
	type Output = Self;
	fn neg(self) -> Self::Output {
	    self.0.neg().into()
	}
}

impl Zero for BFloat {
	fn is_zero(&self) -> bool {
	    self.0.is_zero()
	}
	fn zero() -> Self {
	    0.into()
	}
	fn set_zero(&mut self) {
	    *self = Self::zero()
	}
}

impl One for BFloat {
	fn is_one(&self) -> bool
	    where
	        Self: PartialEq, {
	    self.0 == Self::from(1).0
	}
	fn one() -> Self {
	    1.into()
	}
	fn set_one(&mut self) {
	    *self = Self::one()
	}
}

impl From<BigFloat> for BFloat {
	fn from(value: BigFloat) -> Self {
	    Self(value)
	}
}

impl_from!(
	f32, f64,
	u8, u16, u32, u64, u128,
	i8, i16, i32, i64, i128
);

impl BFloat {
	pub fn as_decimal_string(&self) -> String {
		utils::bigfloat_to_plain_decimal(self).unwrap()
	}
}
