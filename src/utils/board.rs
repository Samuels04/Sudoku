use std::{collections::HashMap, fmt::Display};
use super::{Square, Number};



/// Represents a Board. Instead of using a Matrix, I decided to use a 
/// Hash Map for the sake of simplicity.
/// ## Methods
/// * **new**: instantiate a new board with the provided rows and columns
/// * **get_square**: returns the object of class square at any given index
/// * **set_square**: sets the value of the square to any given option of the enumeration Number
/// * **get_row**: returns the row of any given square
/// * **get_col**: returns the column of any given square
/// 
#[allow(dead_code)]
pub struct Board {
	rows: i8,
	cols: i8,
	size: usize,
	board: HashMap<i8, Square>,
}
#[allow(dead_code)]
impl Board {
	pub fn new(rows: i8, cols: i8) -> Self {
		let size = usize::try_from(rows*cols).unwrap();
		Board { rows: rows, cols: cols, size: size, board: HashMap::with_capacity(size)}
	}

	
	/// .
	pub fn get_square(&mut self, index: i8) -> Option<&mut Square> {
		return match self.board.get_mut(&index){
			Some(square) => Some(square),
			None => None,
		}
	}

	/// Sets the square of this [`Board`].
	///
	/// # Panics
	///
	/// Panics if the index is not valid
	pub fn set_square(&mut self, index: i8, value: Number) {
		if index <= 0 || index > i8::try_from(self.size).unwrap() {
			panic!("Invalid index");
		}
		self.board.get_mut(&index).unwrap().set_value(&value);
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

	pub fn get_col(&self, of: &Square) -> i8 {
		if let Some(index) = self.board.iter().find_map(|(key, val)| if val == of { Some(key) } else { None }) {
			return index % 9;
		}

		eprintln!("Error: Value not found in the board");

		0
	}

	pub fn set_row(&mut self,row: i8, cols: Vec<i8>, values: Vec<Number>) {
	
		let mut current_index = 9*(row - 1) + 1;
		let end_index_of_row = 9*row;

		let mut col = 0;

		while current_index <= end_index_of_row {
			if cols.contains(&current_index) {
				self.board.get_mut(&current_index).unwrap().set_value(values.get(col).unwrap());
				col += 1;
			}
			current_index += 1;
		}

	}

	pub fn is_number_in_row(&mut self, number: Number, row: i8) -> bool{
		let mut current_index = 9*(row - 1) + 1;
		let end_index_of_row = 9*row;

		while current_index != end_index_of_row{
			if self.get_square(current_index).unwrap().get_value() == number{
				return true;
			}
			current_index += 1;
		}

		return false;
		
	}

	pub fn is_number_in_col(&mut self, number: Number, col: i8) -> bool {
		let mut current_index = col.clone();
		let end_index_of_col = 72 + col;

		while current_index != end_index_of_col {
			if self.get_square(current_index).unwrap().get_value() == number {
				return true
			}
			current_index += 9;
		}

		return false
	}

	pub fn is_number_in_subsquare(&mut self, number: Number, position: (i8, i8)) -> bool {
		let index = position.0*position.1;

		
	}

	fn is_edge(&self, square: Square) -> bool{
		let ind = *self.board.iter().find_map(|(key, &val)| if val == square { Some(key) } else { None }).unwrap();
		if ind <= 0 || ind > i8::try_from(self.size).unwrap() {
			panic!("Invalid index");
		}

		let edge_top = ind >= 1 && ind <= 0;
		let edge_left = ind % 9 == 1;
		let edge_right = ind % 9 == 0;
		let edge_bottom = ind >= 73 && ind <= 81;

		return edge_bottom || edge_left || edge_right || edge_top
	}

	pub fn print(&self) {
		for i in self.board.iter() {
			if self.is_edge(*i.1) {
				println!("{:?}\n", i.1)
			}
			else
			{
				print!("{:?}", i.1)
			}

		}
	}

	fn get_subsquare(&self, square: &Square) -> i8{
		let row = self.get_row(square);
		let col = self.get_col(&square);
		let mut subsquare: i8;
		match row{
			1..=3 => match col {
				1..=3 => subsquare = 1,
				4..=6 => subsquare = 2,
				_ => subsquare = 3,
			},
			4..=6 => match col {
				1..=3 => subsquare = 4,
				4..=6 => subsquare = 5,
				_ => subsquare = 6,
			}
			_ => match col {
				1..=3 => subsquare = 7,
				4..=6 => subsquare = 8,
				_ => subsquare = 9,
			}
		}

		subsquare
	}

	pub fn empty_square(&mut self, index: i8) {
		self.board.get_mut(&index).unwrap().set_value(&Number::EMPTY);
	}

}

impl Iterator for Board {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
       Some(*self.board.iter().next().unwrap().1)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.fold(
            0,
            |count, _| count + 1,
        )
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        #[inline]
        fn some<T>(_: Option<T>, x: T) -> Option<T> {
            Some(x)
        }

        self.fold(None, some)
	}
    
}

impl Clone for Board {
    fn clone(&self) -> Self {
        Self { rows: self.rows.clone(), cols: self.cols.clone(), board: self.board.clone(), size: self.size}
    }
}

