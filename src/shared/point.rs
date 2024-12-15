#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn slope(&self, other: &Point) -> f32 {
        (other.y - self.y) as f32 / (other.x - self.x) as f32
    }

    pub fn manhattan(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn distance(&self, other: &Point) -> f32 {
        let ((x1, y1), (x2, y2)) = (
            (self.x as f32, self.y as f32),
            (other.x as f32, other.y as f32),
        );
        f32::sqrt((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0))
    }

    pub fn in_bounds(&self, min: Point, max: Point) -> bool {
        self.x >= min.x && self.x <= max.x && self.y >= min.y && self.y <= max.y
    }

    pub fn get_normal_vector(&self, other: &Point) -> (f32, f32) {
        let dist = self.distance(other);
        (self.x as f32 / dist, self.y as f32 / dist)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
