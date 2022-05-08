extern crate peg;
use crate::sudoku_value::SudokuValue;
use crate::sudoku_state::SudokuState;

peg::parser! {
    grammar sudoku_parser() for str {
        rule filled_cell() -> SudokuValue
            = n:$(['0'..='9']) { SudokuValue::from_digit(n.chars().next().unwrap()) }
        rule empty_cell() -> SudokuValue
            = "_" { SudokuValue::Empty }
        rule sudoku_value() -> SudokuValue
            = n:(filled_cell() / empty_cell()) { n }
        rule sudoku_horizontal_line() -> [SudokuValue;9]
            = values:sudoku_value() **<9,9> " " {
                values.try_into().unwrap()
            }
        pub rule parse() -> SudokuState
            = lines:sudoku_horizontal_line() **<9,9> "\n" "\n" * {
                let values = lines.try_into().unwrap();
                SudokuState{values}
            }
    }
}

impl SudokuState {
    /// Generates a new Day22Setup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new Day22Setup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> SudokuState {
        sudoku_parser::parse(input_str).unwrap()
    }
}
