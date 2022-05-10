use itertools::Itertools;
#[derive(Debug, Clone, Copy)]
pub struct Point(pub u8, pub u8);

impl Point {
    pub fn all_points() -> Box<dyn Iterator<Item = Point>> {
        Box::new(
            (0..9)
                .cartesian_product(0..9)
                .map(|(i, j)| -> Point { Point(i, j) }),
        )
    }

    pub fn get_horizontal_lines() -> Box<dyn Iterator<Item = [Point; 9]>> {
        Box::new((0..9).map(|y| {
            (0..9)
                .map(|x| Point(x, y))
                .collect::<Vec<Point>>()
                .try_into()
                .unwrap()
        }))
    }

    pub fn get_vertical_lines() -> Box<dyn Iterator<Item = [Point; 9]>> {
        Box::new((0..9).map(|x| {
            (0..9)
                .map(|y| Point(x, y))
                .collect::<Vec<Point>>()
                .try_into()
                .unwrap()
        }))
    }

    pub fn get_blocks() -> Box<dyn Iterator<Item = [Point; 9]>> {
        Box::new((0..3).cartesian_product(0..3).map(|(start_x, start_y)| {
            (0..3)
                .cartesian_product(0..3)
                .map(|(x, y)| Point(x + start_x * 3, y + start_y * 3))
                .collect::<Vec<Point>>()
                .try_into()
                .unwrap()
        }))
    }

    pub fn get_horizontal_matching<'a>(&'a self) -> Box<dyn Iterator<Item = Point> + 'a> {
        Box::new((0..9).map(|i| Point(i, self.1)))
    }

    pub fn get_vertical_matching<'a>(&'a self) -> Box<dyn Iterator<Item = Point> + 'a> {
        Box::new((0..9).map(|i| Point(self.0, i)))
    }
    pub fn get_block_matching<'a>(&'a self) -> Box<dyn Iterator<Item = Point> + 'a> {
        let rounded_x = self.0 / 3 * 3;
        let rounded_y = self.1 / 3 * 3;
        Box::new(
            (0..3)
                .cartesian_product(0..3)
                .map(move |(x, y)| Point(x + rounded_x, y + rounded_y)),
        )
    }
}
