use sudoku_solver::SudokuState;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input_data.txt").expect("Need input_data.txt to exist");
    let mut input_str: String = String::new();
    f.read_to_string(&mut input_str).unwrap();
    let mut initial = SudokuState::new(&input_str[..]);
    println!("Initial sudoku state: \n{}", initial);
    let solution = initial.solve();
    println!("Final sudoku state: \n{}", solution);
}
