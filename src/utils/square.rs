use super::Number;

pub struct Square {
	value: Number,
}

impl Square {
	pub fn new(value: Number) -> Self {
		Square{value: value}
	}

	pub fn set_value(&self, to: Number) {
		self.value = to;
	}

	pub fn is_empty(&self) -> bool {
		self.value == Number::EMPTY
	}
}

impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}