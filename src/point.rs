use itertools::Itertools;
#[derive(Debug, Clone, Copy)]
pub struct SudokuPoint(pub u8, pub u8);

impl SudokuPoint {
    pub fn all_points() -> Box<dyn Iterator<Item = SudokuPoint>> {
        Box::new(
            (0..9)
                .cartesian_product(0..9)
                .map(|(i, j)| -> SudokuPoint { SudokuPoint(i, j) }),
        )
    }

    pub fn get_horizontal_lines() -> Box<dyn Iterator<Item = [SudokuPoint; 9]>> {
        Box::new((0..9).map(|y| {
            (0..9)
                .map(|x| SudokuPoint(x, y))
                .collect::<Vec<SudokuPoint>>()
                .try_into()
                .unwrap()
        }))
    }

    pub fn get_vertical_lines() -> Box<dyn Iterator<Item = [SudokuPoint; 9]>> {
        Box::new((0..9).map(|x| {
            (0..9)
                .map(|y| SudokuPoint(x, y))
                .collect::<Vec<SudokuPoint>>()
                .try_into()
                .unwrap()
        }))
    }

    pub fn get_blocks() -> Box<dyn Iterator<Item = [SudokuPoint; 9]>> {
        Box::new((0..3).cartesian_product(0..3).map(|(start_x, start_y)| {
            (0..3)
                .cartesian_product(0..3)
                .map(|(x, y)| SudokuPoint(x + start_x * 3, y + start_y * 3))
                .collect::<Vec<SudokuPoint>>()
                .try_into()
                .unwrap()
        }))
    }

    pub fn get_horizontal_matching(&self) -> Box<dyn Iterator<Item = SudokuPoint> + '_> {
        Box::new((0..9).map(|i| SudokuPoint(i, self.1)))
    }

    pub fn get_vertical_matching(&self) -> Box<dyn Iterator<Item = SudokuPoint> + '_> {
        Box::new((0..9).map(|i| SudokuPoint(self.0, i)))
    }

    pub fn get_block_matching(&self) -> Box<dyn Iterator<Item = SudokuPoint> + '_> {
        let rounded_x = self.0 / 3 * 3;
        let rounded_y = self.1 / 3 * 3;
        Box::new(
            (0..3)
                .cartesian_product(0..3)
                .map(move |(x, y)| SudokuPoint(x + rounded_x, y + rounded_y)),
        )
    }
}
