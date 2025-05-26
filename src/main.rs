use std::{fs, str::FromStr};

use sudoku::Board;

fn main() -> Result<(), String> {
    let contents = fs::read_to_string("./fixtures/57.csv").map_err(|e| format!("{}", e))?;
    let board = Board::from_str(&contents)?;
    println!("{}", board);
    if let Err(violations) = board.check_solution() {
        for violation in violations {
            println!("{violation}");
        }
        println!("Not solved!");
    } else {
        println!("Solved!");
    }
    Ok(())
}
