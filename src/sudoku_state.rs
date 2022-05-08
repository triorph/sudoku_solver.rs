use crate::sudoku_value::SudokuValue;

#[derive(Clone)]
pub struct SudokuState {
    pub values: [[SudokuValue; 9]; 9],
}

impl std::fmt::Display for SudokuState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        for i in 0..9 {
            if i % 3 == 0 {
                ret += "+-------+-------+-------+\n";
            }
            for j in 0..9 {
                if j % 3 == 0 {
                    ret += "| ";
                }
                ret += &format!("{}", self.values[i][j])[..];
                ret += " ";
            }
            ret += "|\n";
        }
                ret += "+-------+-------+-------+\n";
        write!(f, "{}", ret)
    }
}

impl SudokuState {
    pub fn solve(&self) -> SudokuState {
        self.clone()
    }
}
