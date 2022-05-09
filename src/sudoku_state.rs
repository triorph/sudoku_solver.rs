use crate::point::Point;
use crate::sudoku_value::SudokuValue;
use itertools::Itertools;

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
                ret += &format!("{}", self.values[i][j]);
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
        let mut solution = self.clone();
        solution.reduce_while_you_can();
        solution
    }

    fn reduce_while_you_can(&mut self) {
        loop {
            let start_count = self.empty_count();
            self.reduce_once();
            if self.empty_count() == start_count {
                break;
            }
        }
    }

    fn reduce_once(&mut self) {
        for point in Point::all_points().iter() {
            self.reduce_at_point(point)
        }
        self.reduce_horizontal_lines();
        self.reduce_vertical_lines();
        self.reduce_blocks();
    }

    fn reduce_horizontal_lines(&mut self) {
        for line in self.get_horizontal_lines().into_iter() {
            self.reduce_ruled_points(line)
        }
    }

    fn reduce_vertical_lines(&mut self) {
        for line in self.get_vertical_lines().into_iter() {
            self.reduce_ruled_points(line)
        }
    }

    fn reduce_blocks(&mut self) {
        for block in self.get_blocks().into_iter() {
            self.reduce_ruled_points(block)
        }
    }

    fn get_horizontal_lines(&self) -> [[Point; 9]; 9] {
        (0..9)
            .map(|y| {
                (0..9)
                    .map(|x| Point(x, y))
                    .collect::<Vec<Point>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[Point; 9]>>()
            .try_into()
            .unwrap()
    }

    fn get_vertical_lines(&self) -> [[Point; 9]; 9] {
        (0..9)
            .map(|x| {
                (0..9)
                    .map(|y| Point(x, y))
                    .collect::<Vec<Point>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[Point; 9]>>()
            .try_into()
            .unwrap()
    }

    fn get_blocks(&self) -> [[Point; 9]; 9] {
        (0..3)
            .cartesian_product(0..3)
            .map(|(start_x, start_y)| {
                (0..3)
                    .cartesian_product(0..3)
                    .map(|(x, y)| Point(x + start_x * 3, y + start_y * 3))
                    .collect::<Vec<Point>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[Point; 9]>>()
            .try_into()
            .unwrap()
    }

    fn reduce_ruled_points(&mut self, points: [Point; 9]) {
        // The idea of this method is to find a location where in this set of points, only 1 number
        // fits the rules. e.g. if 3 points can be 1 8 9; 8 9; and 8 9; respectively, then even
        // though the "simple reduce" gives us 1 8 9, we want to be able to discern that 1 can only
        // be assigned to that point.
        for value in SudokuValue::all_values().into_iter() {
            let matching = self.get_points_that_can_be_value(&points, value);
            if matching.len() == 1 {
                self.set_at_point(&matching[0], value)
            }
        }
    }

    fn get_points_that_can_be_value(&self, points: &[Point; 9], value: SudokuValue) -> Vec<Point> {
        points
            .iter()
            .filter(|p| self.find_values_at_point(p).contains(&value))
            .cloned()
            .collect()
    }

    fn reduce_at_point(&mut self, point: &Point) {
        if self.get_from_point(point) == SudokuValue::Empty {
            let values = self.find_values_at_point(point);
            if values.len() == 1 {
                self.set_at_point(point, values[0]);
            }
        }
    }

    fn find_values_at_point(&self, point: &Point) -> Vec<SudokuValue> {
        if self.get_from_point(point) != SudokuValue::Empty {
            return vec![];
        }
        let horizontal = self.find_horizontal_matching_point(point);
        let vertical = self.find_vertical_matching_point(point);
        let block = self.find_block_matching_point(point);
        let mut ret: Vec<SudokuValue> = vec![];
        for value in SudokuValue::all_values().iter() {
            if !(horizontal.contains(value) | vertical.contains(value) | block.contains(value)) {
                ret.push(*value)
            }
        }
        ret
    }

    fn find_horizontal_matching_point(&self, point: &Point) -> [SudokuValue; 9] {
        (0..9)
            .map(|i| Point(i, point.1))
            .map(|p| self.get_from_point(&p))
            .collect::<Vec<SudokuValue>>()
            .try_into()
            .unwrap()
    }

    fn find_vertical_matching_point(&self, point: &Point) -> [SudokuValue; 9] {
        (0..9)
            .map(|i| Point(point.0, i))
            .map(|p| self.get_from_point(&p))
            .collect::<Vec<SudokuValue>>()
            .try_into()
            .unwrap()
    }

    fn find_block_matching_point(&self, point: &Point) -> [SudokuValue; 9] {
        let rounded_x = point.0 / 3 * 3;
        let rounded_y = point.1 / 3 * 3;
        (0..3)
            .cartesian_product(0..3)
            .map(|(x, y)| Point(x + rounded_x, y + rounded_y))
            .map(|p| self.get_from_point(&p))
            .collect::<Vec<SudokuValue>>()
            .try_into()
            .unwrap()
    }

    fn get_from_point(&self, point: &Point) -> SudokuValue {
        self.values[point.1 as usize][point.0 as usize]
    }

    fn set_at_point(&mut self, point: &Point, value: SudokuValue) {
        self.values[point.1 as usize][point.0 as usize] = value;
    }

    fn empty_count(&self) -> usize {
        Point::all_points()
            .into_iter()
            .filter(|p| self.get_from_point(p) == SudokuValue::Empty)
            .count()
    }
}

#[cfg(test)]
mod test {
    use crate::point::Point;
    use crate::sudoku_value::SudokuValue;
    use crate::SudokuState;

    #[test]
    fn test_find_values_at_point() {
        let input_str = include_str!("../test_data.txt");
        let state = SudokuState::new(input_str);
        assert_eq!(
            state.find_values_at_point(&Point(2, 8)),
            vec![SudokuValue::Four, SudokuValue::Six, SudokuValue::Nine]
        );
    }

    #[test]
    fn test_reduce_at_point() {
        let input_str = include_str!("../test_data.txt");
        let mut state = SudokuState::new(input_str);
        state.reduce_at_point(&Point(4, 6));
        assert_eq!(state.get_from_point(&Point(4, 6)), SudokuValue::Nine);
    }

    #[test]
    fn test_reduce_blocks() {
        let input_str = include_str!("../test_data.txt");
        let mut state = SudokuState::new(input_str);
        state.reduce_blocks();
        assert_eq!(state.get_from_point(&Point(1, 7)), SudokuValue::One);
    }

    #[test]
    fn test_further_reduction() {
        let input_str = include_str!("../test_data.txt");
        let mut state = SudokuState::new(input_str);
        for _ in 0..3 {
            state.reduce_once();
        }
        state.reduce_ruled_points(state.get_blocks()[5]);
        assert_eq!(state.get_from_point(&Point(3, 6)), SudokuValue::Two);
    }
}
