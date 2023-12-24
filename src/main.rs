use utils::Board;

mod utils;
fn main() {

    let mut board: Board;
    let mut amount_of_numbers: i8;
    for i in 1..10 {
        println!("What are the numbers in this row?");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input);
        let numbers: Vec<&str> = input.trim().split(",").collect();

        println!("In which columns are they?");
        let mut input2 = String::new();
        std::io::stdin().read_line(&mut input2);
        

        let cols: Vec<&str> = input2.trim().split(",").collect();
        
        let mut counter: usize = 0;
        for ii in 0..10 {
            if cols.contains(&format!("{}", ii).as_str()) {
                board.get_mut(i).unwrap().get_mut(ii) = numbers.get(counter);
                counter+= 1;
                
            }
        }
        
    }
}

