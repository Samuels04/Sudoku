use strum::IntoEnumIterator;
use utils::{Board, Square};

//use binary_tree::{BinaryTree, count::CountTree};

use crate::utils::Number;

mod utils;
mod tree;
#[allow(unused_must_use, unused_variables)]
fn main() {

    let mut board = Board::new(9, 9);

    set_initial_values(&mut board);

    for i in Number::iter() {

        for ii in 0..=9 {
            if board.is_number_in_row(i, ii) {
                continue;
            }

        }
    }
    
}

#[allow(unused_must_use, unused_variables)]
fn set_initial_values(board: &mut Board) {
    for i in 1..=9 {
        println!("What are the numbers in this row?");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input);
        let numbers_as_str: Vec<&str> = input.trim().split(",").collect();

        let mut numbers: Vec<Number> = Vec::new();

        for i in numbers_as_str {
            numbers.push(Number::parse(i.to_string()));
        }

        println!("In which columns are they?");
        let mut input2 = String::new();
        std::io::stdin().read_line(&mut input2);
        let cols: Vec<&str> = input2.trim().split("\n,").collect();

        board.set_row(i, cols, numbers);
        
        
        
    }
}

fn check_value(board: &mut Board, square: Square) -> bool {
    let row = board.get_row(&square);

    let col = board.get_col(&square);

    if board.is_number_in_row(square.get_value(), row) || board.is_number_in_col(square.get_value(), col) || board.is_number_in_diag(square.get_value(), (row, col)){
        return false
    }

    return true

}

