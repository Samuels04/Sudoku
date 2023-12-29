

use strum::IntoEnumIterator;
use utils::{Board, Square};

//use binary_tree::{BinaryTree, count::CountTree};

use crate::utils::Number;

mod utils;
#[allow(unused_must_use, unused_variables)]
fn main() {

    let mut board = Board::new(9, 9);

    set_initial_values(&mut board);

    let solved = solve(&mut board, 1);
    
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

fn check_value(board: &mut Board, index: i8, number: Number) -> bool {
    let square = Square::new(number);
    /*
    if board.is_number_in_row(square.get_value(), row) || board.is_number_in_col(square.get_value(), col) || board.is_number_in_subsquare(square.get_value(), (row, col)){
        return false
    }
    */
    return true

}

fn solve(board: &mut Board, index: i8) -> bool{
    let mut current_square = board.get_square(index);
    if index == 81 {
        return true
    }
    else if current_square.is_some() {
        return solve(board, index + 1)
    }
    else {
        for i in Number::iter() {
           if check_value(board, index, i) {
                board.set_square(index, i);
                if solve(board, index + 1) {
                    return true
                }
                board.empty_square(index)
           }
        }
        return false //Backtrack
    }
}

