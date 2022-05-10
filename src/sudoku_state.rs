use crate::point::Point;
use crate::sudoku_value::SudokuValue;
use std::collections::HashSet;

#[derive(Debug)]
pub struct SudokuError();

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
    pub fn solve(&self) -> Result<Vec<SudokuState>, SudokuError> {
        let mut solution = self.clone();
        solution.reduce_while_you_can()?;
        if solution.empty_count() > 0 {
            self.split_solutions()
        } else {
            Ok(vec![solution])
        }
    }

    fn split_solutions(&self) -> Result<Vec<SudokuState>, SudokuError> {
        let point_to_split = self.find_best_split_point();
        let mut ret = vec![];
        for value in self
            .find_allowed_values_at_point(&point_to_split)
            .into_iter()
        {
            let mut clone = self.clone();
            clone.set(&point_to_split, value);
            if let Ok(solutions) = clone.solve() {
                ret.extend(solutions);
            }
        }
        if !ret.is_empty() {
            Ok(ret)
        } else {
            Err(SudokuError())
        }
    }

    fn find_best_split_point(&self) -> Point {
        Point::all_points()
            .reduce(|p1, p2| {
                if self.get(&p1) != SudokuValue::Empty {
                    p2
                } else if self.get(&p2) != SudokuValue::Empty
                    || self.find_allowed_values_at_point(&p1).len()
                        < self.find_allowed_values_at_point(&p2).len()
                {
                    p1
                } else {
                    p2
                }
            })
            .unwrap()
    }

    fn reduce_while_you_can(&mut self) -> Result<(), SudokuError> {
        loop {
            let start_count = self.empty_count();
            self.reduce_once()?;
            if self.empty_count() == start_count {
                break;
            }
        }
        Ok(())
    }

    fn reduce_once(&mut self) -> Result<(), SudokuError> {
        for point in Point::all_points() {
            self.reduce_at_point(&point)?
        }
        self.reduce_horizontal_lines();
        self.reduce_vertical_lines();
        self.reduce_blocks();
        Ok(())
    }

    fn reduce_horizontal_lines(&mut self) {
        for line in Point::get_horizontal_lines() {
            self.reduce_ruled_points(line)
        }
    }

    fn reduce_vertical_lines(&mut self) {
        for line in Point::get_vertical_lines() {
            self.reduce_ruled_points(line)
        }
    }

    fn reduce_blocks(&mut self) {
        for block in Point::get_blocks() {
            self.reduce_ruled_points(block)
        }
    }

    fn reduce_ruled_points(&mut self, points: [Point; 9]) {
        // The idea of this method is to find a location where in this set of points, only 1 number
        // fits the rules. e.g. if 3 points can be 1 8 9; 8 9; and 8 9; respectively, then even
        // though the "simple reduce" gives us 1 8 9, we want to be able to discern that 1 can only
        // be assigned to that point.
        let values_found_at_points: [HashSet<SudokuValue>; 9] = points
            .into_iter()
            .map(|p| self.find_allowed_values_at_point(&p))
            .collect::<Vec<HashSet<SudokuValue>>>()
            .try_into()
            .unwrap();
        for value in SudokuValue::all_values().into_iter() {
            let matching =
                self.get_points_that_can_be_value(&points, &values_found_at_points, value);
            if matching.len() == 1 {
                self.set(&matching[0], value)
            }
        }
    }

    fn get_points_that_can_be_value(
        &self,
        points: &[Point; 9],
        values_found_at_points: &[HashSet<SudokuValue>; 9],
        value: SudokuValue,
    ) -> Vec<Point> {
        (0..9)
            .filter(|i| values_found_at_points[*i].contains(&value))
            .map(|i| points[i])
            .collect()
    }

    fn reduce_at_point(&mut self, point: &Point) -> Result<(), SudokuError> {
        if self.get(point) == SudokuValue::Empty {
            let values = self.find_allowed_values_at_point(point);
            if values.len() == 1 {
                self.set(point, values.into_iter().next().unwrap());
            } else if values.is_empty() {
                return Err(SudokuError());
            }
        }
        Ok(())
    }

    fn find_allowed_values_at_point(&self, point: &Point) -> HashSet<SudokuValue> {
        if self.get(point) != SudokuValue::Empty {
            return HashSet::new();
        }
        let horizontal = self.find_horizontal_matching_point(point);
        let vertical = self.find_vertical_matching_point(point);
        let block = self.find_block_matching_point(point);
        let ret: HashSet<SudokuValue> = HashSet::from(SudokuValue::all_values());
        ret.difference(&horizontal)
            .cloned()
            .collect::<HashSet<SudokuValue>>()
            .difference(&vertical)
            .cloned()
            .collect::<HashSet<SudokuValue>>()
            .difference(&block)
            .cloned()
            .collect::<HashSet<SudokuValue>>()
    }

    fn get_values_at_points(
        &self,
        points: Box<dyn Iterator<Item = Point> + '_>,
    ) -> HashSet<SudokuValue> {
        points
            .map(|p| self.get(&p))
            .collect::<HashSet<SudokuValue>>()
    }

    fn find_horizontal_matching_point(&self, point: &Point) -> HashSet<SudokuValue> {
        self.get_values_at_points(point.get_horizontal_matching())
    }

    fn find_vertical_matching_point(&self, point: &Point) -> HashSet<SudokuValue> {
        self.get_values_at_points(point.get_vertical_matching())
    }

    fn find_block_matching_point(&self, point: &Point) -> HashSet<SudokuValue> {
        self.get_values_at_points(point.get_block_matching())
    }

    fn get(&self, point: &Point) -> SudokuValue {
        self.values[point.1 as usize][point.0 as usize]
    }

    fn set(&mut self, point: &Point, value: SudokuValue) {
        self.values[point.1 as usize][point.0 as usize] = value;
    }

    fn empty_count(&self) -> usize {
        Point::all_points()
            .filter(|p| self.get(p) == SudokuValue::Empty)
            .count()
    }
}

#[cfg(test)]
mod test {
    use crate::point::Point;
    use crate::sudoku_state::SudokuError;
    use crate::sudoku_value::SudokuValue;
    use crate::SudokuState;
    use std::collections::HashSet;

    #[test]
    fn test_find_values_at_point() {
        let input_str = include_str!("../test_data.txt");
        let state = SudokuState::new(input_str);
        assert_eq!(
            state.find_allowed_values_at_point(&Point(2, 8)),
            HashSet::from([SudokuValue::Four, SudokuValue::Six, SudokuValue::Nine])
        );
    }

    #[test]
    fn test_reduce_at_point() -> Result<(), SudokuError> {
        let input_str = include_str!("../test_data.txt");
        let mut state = SudokuState::new(input_str);
        state.reduce_at_point(&Point(4, 6))?;
        assert_eq!(state.get(&Point(4, 6)), SudokuValue::Nine);
        Ok(())
    }

    #[test]
    fn test_reduce_blocks() {
        let input_str = include_str!("../test_data.txt");
        let mut state = SudokuState::new(input_str);
        state.reduce_blocks();
        assert_eq!(state.get(&Point(1, 7)), SudokuValue::One);
    }

    #[test]
    fn test_unsolveable_gives_error() {
        let mut input_str = String::new();
        input_str += "1 2 3 4 5 6 7 8 _\n";
        input_str += "_ _ _ _ _ _ _ _ 9\n";
        input_str += "_ _ _ _ _ _ _ _ _\n";
        input_str += "_ _ _ _ _ _ _ _ _\n";
        input_str += "_ _ _ _ _ _ _ _ _\n";
        input_str += "_ _ _ _ _ _ _ _ _\n";
        input_str += "_ _ _ _ _ _ _ _ _\n";
        input_str += "_ _ _ _ _ _ _ _ _\n";
        input_str += "_ _ _ _ _ _ _ _ _\n";
        let state = SudokuState::new(&input_str);
        let solution = state.solve();
        assert!(solution.is_err());
    }

    #[test]
    fn test_simple_solve() -> Result<(), SudokuError> {
        // A sudoku described as "simple" by the sudoku book I have.
        // Only requires the basic "reduce" strategy to solve.
        let input_str = include_str!("../test_data.txt");
        let state = SudokuState::new(input_str);
        let solution = state.solve()?;
        assert_eq!(solution.len(), 1);
        let solution = &solution[0];
        assert_eq!(solution.empty_count(), 0);
        let mut expected_solution = String::new();
        expected_solution += "+-------+-------+-------+\n";
        expected_solution += "| 8 5 9 | 1 4 2 | 6 7 3 |\n";
        expected_solution += "| 4 3 2 | 6 7 5 | 8 1 9 |\n";
        expected_solution += "| 7 6 1 | 8 3 9 | 2 5 4 |\n";
        expected_solution += "+-------+-------+-------+\n";
        expected_solution += "| 6 2 4 | 3 8 1 | 5 9 7 |\n";
        expected_solution += "| 1 7 8 | 9 5 6 | 3 4 2 |\n";
        expected_solution += "| 5 9 3 | 4 2 7 | 1 8 6 |\n";
        expected_solution += "+-------+-------+-------+\n";
        expected_solution += "| 3 4 5 | 2 9 8 | 7 6 1 |\n";
        expected_solution += "| 9 1 7 | 5 6 3 | 4 2 8 |\n";
        expected_solution += "| 2 8 6 | 7 1 4 | 9 3 5 |\n";
        expected_solution += "+-------+-------+-------+\n";
        assert_eq!(format!("{}", solution), expected_solution);
        Ok(())
    }

    #[test]
    fn test_diabolical_solve() -> Result<(), SudokuError> {
        // A sudoku described as "diabolical" by thu sudoku book I have
        // Requires more than just reduce to solve
        let input_str = include_str!("../test_data_2.txt");
        let state = SudokuState::new(input_str);
        let solution = state.solve()?;
        assert_eq!(solution.len(), 1);
        let solution = &solution[0];
        assert_eq!(solution.empty_count(), 0);
        let mut expected_solution = String::new();
        expected_solution += "+-------+-------+-------+\n";
        expected_solution += "| 3 9 1 | 6 4 2 | 5 7 8 |\n";
        expected_solution += "| 6 2 8 | 3 5 7 | 9 1 4 |\n";
        expected_solution += "| 7 4 5 | 9 1 8 | 2 3 6 |\n";
        expected_solution += "+-------+-------+-------+\n";
        expected_solution += "| 2 7 9 | 5 8 3 | 6 4 1 |\n";
        expected_solution += "| 1 3 6 | 7 2 4 | 8 5 9 |\n";
        expected_solution += "| 8 5 4 | 1 9 6 | 7 2 3 |\n";
        expected_solution += "+-------+-------+-------+\n";
        expected_solution += "| 9 8 2 | 4 7 1 | 3 6 5 |\n";
        expected_solution += "| 4 6 7 | 8 3 5 | 1 9 2 |\n";
        expected_solution += "| 5 1 3 | 2 6 9 | 4 8 7 |\n";
        expected_solution += "+-------+-------+-------+\n";
        assert_eq!(format!("{}", solution), expected_solution);
        Ok(())
    }
}
