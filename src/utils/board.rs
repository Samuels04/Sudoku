use std::collections::HashMap;
use super::{Square, Number};

#[allow(dead_code)]
pub struct Board {
	rows: i8,
	cols: i8,
	board: HashMap<i8, Square>,
}
#[allow(dead_code)]
impl Board {
	pub fn new(rows: i8, cols: i8) -> Self {
		let size = usize::try_from(rows*cols).unwrap();
		Board { rows: rows, cols: cols, board: HashMap::with_capacity(size)}
	}
	
	pub fn get_square(&mut self, index: i8) -> Option<&mut Square> {
		self.board.get_mut(&index)
	}

	pub fn set_square(&mut self, index: i8, value: Number) {
		let previous_square = self.board.get_mut(&index).unwrap();
		previous_square.set_value(&value);
	}

	pub fn get_row(&self, of: &Square) -> i8 {
		if let Some(index) = self.board.iter().find_map(|(key, val)| if val == of { Some(key) } else { None }) {
			return index / 9;
		}
	
		// Handle the case when the value is not found
		eprintln!("Error: Value not found in the board");
		// Choose an appropriate default value or handle the situation accordingly
		0
	}

	pub fn set_row(&mut self,row: i8, cols: Vec<&str>, values: Vec<Number>) {
		let mut cols_i: Vec<i8> = Vec::new();
		for &i in cols.iter() {
			cols_i.push(i.parse().unwrap())
		}

		let mut current_index = 9*(row - 1) + 1;
		let end_index_of_row = 9*row;

		let mut col = 0;

		while current_index <= end_index_of_row {
			if cols_i.contains(&current_index) {
				self.board.get_mut(&current_index).unwrap().set_value(values.get(col).unwrap());
				col += 1;
			}
			current_index += 1;
		}

	}

}