use itertools::Itertools;
#[derive(Debug, Clone, Copy)]
pub struct Point(pub u8, pub u8);

impl Point {
    pub fn all_points() -> Vec<Point> {
        (0..9)
            .cartesian_product(0..9)
            .map(|(i, j)| -> Point { Point(i, j) })
            .collect()
    }
}
