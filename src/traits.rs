use slint::SharedString;
use alloc::vec::Vec;

pub trait MapToSharedStringVec {
	fn to_shared_string_vec(&self) -> Vec<SharedString>;
}
