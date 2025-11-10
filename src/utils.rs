use crate::types::BFloat;
use alloc::{ string::{ String, ToString }, vec::Vec };

use astro_float::{Consts, Radix, RoundingMode, Sign};
use slint::{ModelRc, SharedString, VecModel};

#[inline]
pub fn vec_to_model(vec: Vec<SharedString>) -> ModelRc<SharedString> {
	ModelRc::new(VecModel::from(vec))
}

pub fn bigfloat_to_plain_decimal(n: &BFloat) -> Result<String, astro_float::Error> {
    // consts cache required by convert_to_radix
    let mut cc = Consts::new().expect("Consts::new() failed");

    // convert to base-10 digits + exponent
    let (sign, digits, exp) = n.0.convert_to_radix(Radix::Dec, RoundingMode::None, &mut cc)?;

    // special cases
    if n.0.is_nan() { return Ok("NaN".to_string()); }
    if n.0.is_inf_pos() { return Ok("Inf".to_string()); }
    if n.0.is_inf_neg() { return Ok("-Inf".to_string()); }
    if n.0.is_zero() { return Ok("0".to_string()); }

    // join digits into string (most-significant first)
    let mut digits_str = String::with_capacity(digits.len());
    for &d in &digits {
        digits_str.push((b'0' + d) as char);
    }

    // exponent `exp` is the number of integer digits the value should have
    // (value = int(digits_str) * 10^(exp - digits.len()))
    let pos: i128 = exp as i128;        // integer-digit count
    let mut s = String::new();

    if sign == Sign::Neg {
        s.push('-');
    }

    if pos <= 0 {
        // value < 1 : "0." + (-pos) zeros + digits
        s.push('0');
        s.push('.');
        for _ in 0..(-pos) {
            s.push('0');
        }
        s.push_str(&digits_str);
    } else {
        let pos_usize = pos as usize;
        if (pos as usize) >= digits_str.len() {
            // all digits fall into integer part, pad with zeros to reach `pos`
            s.push_str(&digits_str);
            for _ in 0..(pos_usize - digits_str.len()) {
                s.push('0');
            }
        } else {
            // split digits into integer and fractional parts at index `pos`
            let (int_part, frac_part) = digits_str.split_at(pos_usize);
            s.push_str(int_part);
            s.push('.');
            s.push_str(frac_part);
        }
    }

    // trim trailing zeros in fractional part and remove trailing dot
    if s.rfind('.').is_some() {
        while s.ends_with('0') {
            s.pop();
        }
        if s.ends_with('.') {
            s.pop();
        }
        // if ends up as just "-", fallback to "0"
        if s == "-" { s = "0".to_string(); }
    }

    Ok(s)
}
