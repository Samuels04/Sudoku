use std::collections::HashMap;
use super::Square;

pub struct Board {
	rows: i8,
	cols: i8,
	board: HashMap<i8, Square>,
}

impl Board {
	pub fn new(rows: i8, cols: i8) -> Self {
		let size = usize::try_from(rows*cols).unwrap();
		Board { rows: rows, cols: cols, board: HashMap::with_capacity(size)}
	}
	
	pub fn get_square(&self, index: i8) -> Option<&mut Square> {
		self.board.get_mut(&index)
	}

	pub fn get_row(&self, of: Square) -> i8 {
		*self.board.iter()
			.find_map(|(key, &val)| if val == of { Some(key) } else { None }).unwrap()
	}
}