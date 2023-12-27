use utils::Board;

use crate::utils::Number;

mod utils;
#[allow(unused_must_use, unused_variables)]
fn main() {

    let board = Board::new(9, 9);

    set_initial_values(board);

    
    
}

#[allow(unused_must_use, unused_variables)]
fn set_initial_values(mut board: Board) {
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

