#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SudokuValue {
    Empty,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl SudokuValue {
    pub fn from_digit(c: char) -> SudokuValue {
        match c {
            '1' => SudokuValue::One,
            '2' => SudokuValue::Two,
            '3' => SudokuValue::Three,
            '4' => SudokuValue::Four,
            '5' => SudokuValue::Five,
            '6' => SudokuValue::Six,
            '7' => SudokuValue::Seven,
            '8' => SudokuValue::Eight,
            '9' => SudokuValue::Nine,
            _ => SudokuValue::Empty,
        }
    }

    pub fn all_values() -> [SudokuValue; 9] {
        return [
            SudokuValue::One,
            SudokuValue::Two,
            SudokuValue::Three,
            SudokuValue::Four,
            SudokuValue::Five,
            SudokuValue::Six,
            SudokuValue::Seven,
            SudokuValue::Eight,
            SudokuValue::Nine,
        ];
    }
}

impl std::fmt::Display for SudokuValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        ret += match self {
            SudokuValue::One => "1",
            SudokuValue::Two => "2",
            SudokuValue::Three => "3",
            SudokuValue::Four => "4",
            SudokuValue::Five => "5",
            SudokuValue::Six => "6",
            SudokuValue::Seven => "7",
            SudokuValue::Eight => "8",
            SudokuValue::Nine => "9",
            SudokuValue::Empty => "_",
        };
        write!(f, "{}", ret)
    }
}
