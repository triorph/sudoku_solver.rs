use std::fs::File;
use std::io::prelude::*;
use sudoku_solver::{SudokuError, SudokuState};

fn main() -> Result<(), SudokuError> {
    let mut f = File::open("input_data.txt").expect("Need input_data.txt to exist");
    let mut input_str: String = String::new();
    f.read_to_string(&mut input_str)
        .expect("Need input data to be in correct format");
    let initial = SudokuState::new(&input_str);
    println!("Initial sudoku state: \n{}", initial);
    let solutions = initial.solve()?;
    println!("Found {} solution(s)\n", solutions.len());
    for (i, solution) in solutions.iter().enumerate() {
        println!("Solution {}: \n{}", i + 1, solution);
    }
    Ok(())
}
