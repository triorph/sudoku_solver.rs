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
            = values:sudoku_value() **<9,9> " " " " * {
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

#[cfg(test)]
mod test {
    use crate::SudokuState;

    #[test]
    fn test_parse() {
        let mut input_str = String::new();
        for i in 0..9 {
            for j in 0..9 {
                input_str += &format!("{} ", (i + j) % 9 + 1);
            }
            input_str += "\n";
        }
        let state = SudokuState::new(&input_str);
        for i in 0..9 {
            for j in 0..9 {
                assert_eq!(format!("{}", state.values[i][j]), format!("{}", (i + j) % 9 + 1));
            }
        }
    }

    #[test]
    fn test_format_from_file() {
        let lines = include_str!("../test_data.txt");
        let state = SudokuState::new(lines);
        let mut expected_string = String::new();
        expected_string += "+-------+-------+-------+\n";
        expected_string += "| _ _ _ | _ 4 2 | _ _ _ |\n";
        expected_string += "| _ 3 2 | _ 7 _ | 8 _ _ |\n";
        expected_string += "| 7 _ 1 | 8 _ _ | 2 _ 4 |\n";
        expected_string += "+-------+-------+-------+\n";
        expected_string += "| _ _ _ | _ 8 _ | 5 _ 7 |\n";
        expected_string += "| 1 7 8 | _ _ _ | 3 4 2 |\n";
        expected_string += "| 5 _ 3 | _ 2 _ | _ _ _ |\n";
        expected_string += "+-------+-------+-------+\n";
        expected_string += "| 3 _ 5 | _ _ 8 | 7 _ 1 |\n";
        expected_string += "| _ _ 7 | _ 6 _ | 4 2 _ |\n";
        expected_string += "| _ _ _ | 7 1 _ | _ _ _ |\n";
        expected_string += "+-------+-------+-------+\n";
        assert_eq!(format!("{}", state), expected_string);
    }
}
