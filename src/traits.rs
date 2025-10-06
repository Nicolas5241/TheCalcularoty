use slint::SharedString;

pub trait MapToSharedStringVec {
	fn to_shared_string_vec(&self) -> Vec<SharedString>;
}
