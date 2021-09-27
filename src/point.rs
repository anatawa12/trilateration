#[derive(Clone, Copy)]
pub(crate) struct Point(pub f64, pub f64);

impl Point {
    pub const ORIGIN: Point = Point(0.0, 0.0);
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl std::ops::Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point(self.0 / rhs, self.1 / rhs)
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
