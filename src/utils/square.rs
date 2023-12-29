use super::Number;
#[derive(Debug)]
pub struct Square {
	value: Number,
}


impl Square {
	pub fn new(value: Number) -> Self {
		Square{value: value}
	}

	pub fn set_value(&mut self, to: &Number) {
		self.value = *to;
	}

	pub fn get_value(&self) -> Number {
		self.value
	}

	pub fn is_empty(&self) -> bool {
		self.value == Number::EMPTY
	}

	pub fn to_string(&self) -> String {
		return String::from(format!(" -- \n|{:?}|\n -- ", self.value))
	}

}

impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Clone for Square 
{
	fn clone(&self) -> Self {
		Square { value: self.value.clone() }
	}
}

impl Copy for Square{}