use core::{fmt, error::Error, ops::{Add, Div, Mul, Neg, Sub}, str::FromStr};

use alloc::boxed::Box;
use alloc::string::String;

use regex::Regex;
use astro_float::{BigFloat, Radix, RoundingMode, Sign};
use num_traits::{One, Pow, Zero};

use crate::{consts::CONSTS_CACHE, utils};

const PRECISION: usize = 1024;
const ROUNDING_MODE: RoundingMode = RoundingMode::ToEven;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum UnitType {
	Hertz,
	Farad,
	Henry,
	Ohm,
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
	type Err = Box<dyn Error>;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let value: BFloat = BigFloat::parse(s, astro_float::Radix::Dec, PRECISION, ROUNDING_MODE, &mut CONSTS_CACHE.lock()).into();
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

	#[inline]
	pub fn nan() -> Self {
		BFloat(BigFloat::nan(None))
	}
}

impl fmt::Display for BFloat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let reg = Regex::new(r"^(.*?)e(.*?)$").unwrap();
		let res_string = bigfloat_to_string(&self.0, Radix::Dec);
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


fn bigfloat_to_string(x: &BigFloat, radix: Radix) -> String {
    let (sign, digits, exp) = x.convert_to_radix(radix, ROUNDING_MODE, &mut CONSTS_CACHE.lock()).unwrap();

    let mut s = String::new();

    // Add sign if needed
    if let Sign::Neg = sign {
        s.push('-');
    }

    // Convert digits (u8s) to characters
    for (i, &d) in digits.iter().enumerate() {
        if i as i32 == exp {
            // Place decimal point if exponent says so
            s.push('.');
        }
        s.push(char::from(b'0' + d));
    }

    // Handle case where exponent is beyond digits
    if exp as usize >= digits.len() {
        for _ in digits.len()..(exp as usize) {
            s.push('0');
        }
        s.push_str(".0");
    }

    s
}
