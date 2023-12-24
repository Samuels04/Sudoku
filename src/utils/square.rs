use super::Number;

pub struct Square {
	value: Number,
}

impl Square {
	pub fn new(value: Number) -> Self {
		Square{value: value}
	}
}