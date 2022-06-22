#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl From<f64> for Point {
    fn from(input: f64) -> Self {
        Point { x: input, y: input }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

fn main() {
    let p1 = Point::from(1.0);
    let p2: Point = (1.0).into();
    println!("{} {}", p1, p2);
}
