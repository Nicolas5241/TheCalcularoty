#![cfg(not(target_os = "android"))]

#![windows_subsystem = "windows"]

mod not_android;
mod units;
mod types;
mod utils;
mod calculations;
mod traits;
mod consts;
mod conversions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    not_android::run_app()
}

#[cfg(test)]
mod tests {
	use num_traits::{Zero, One};

use crate::types::BFloat;

	fn f(v: f64) -> BFloat {
        BFloat::from(v) // convenience: convert f64 → MyFloat
    }

    #[test]
    fn test_basic_arithmetic() {
        let two = f(2.0);
        let three = f(3.0);

        assert_eq!(two.clone() + three.clone(), f(5.0));
        assert_eq!(two.clone() * three.clone(), f(6.0));
        assert_eq!(three.clone() - two.clone(), f(1.0));
    }

	#[test]
	fn zero_functions() {
		let zero = f(0.0);
		let mut one = f(1.0);
		
		assert_eq!(zero.is_zero(), true);
		assert_eq!(one.is_zero(), false);

		one.set_zero();

		assert_eq!(one.is_zero(), true);
	}

	#[test]
	fn one_functions() {
		let mut zero = f(0.0);
		let one = f(1.0);
		
		assert_eq!(zero.is_one(), false);
		assert_eq!(one.is_one(), true);

		zero.set_one();

		assert_eq!(zero.is_one(), true);
	}
}
