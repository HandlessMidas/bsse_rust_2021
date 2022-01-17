use std::f64::consts::PI;
use std::fmt::{Display, Formatter};

struct Point {
    x: f64,
    y: f64,
}

fn dist(from: &Point, to: &Point) -> f64 {
    f64::sqrt((from.x - to.x) * (from.x - to.x) + ((from.y - to.y) * (from.y - to.y)))
}

struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

struct Circle {
    center: Point,
    radius: f64,
}

enum Figure {
    Circle(Circle),
    Rect(Rect),
}

impl Rect {
    fn contains(&self, p: &Point) -> bool {
        self.x <= p.x && self.y <= p.y && self.x + self.width >= p.x && self.y + self.height >= p.y
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Circle {
    fn contains(&self, point: &Point) -> bool {
        dist(&self.center, point) <= self.radius
    }

    fn area(&self) -> f64 {
        self.radius * self.radius * PI
    }
}

impl Figure {
    fn contains(&self, point: &Point) -> bool {
        match self {
            Figure::Circle(circle) => circle.contains(point),
            Figure::Rect(rect) => rect.contains(point)
        }
    }

    fn area(&self) -> f64 {
        match self {
            Figure::Circle(circle) => circle.area(),
            Figure::Rect(rect) => rect.area()
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rect(x: {}, y: {}, width: {}, height: {})", self.x, self.y, self.width, self.height)
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle(center: {}, radius: {})", self.center, self.radius)
    }
}

impl Display for Figure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Figure::Circle(circle) => circle.fmt(f),
            Figure::Rect(rect) => rect.fmt(f)
        }
    }
}

fn main() {
    let zero_point = Point { x: 0.0 , y: 0.0 };

    let examples = [
        Figure::Circle(Circle { center: Point {x: -1.0, y: -1.0}, radius: 1.0 }),
        Figure::Circle(Circle { center: Point {x: -1.0, y: -1.0}, radius: 2.0 }),
        Figure::Rect(Rect { x: 1.0, y: 2.0, width: 12.0, height: 15.0 }),
        Figure::Rect(Rect { x: -1.0, y: -2.0, width: 100.0, height: 15.0 })
    ];

    for x in examples {
        println!("{0}, area = {1}, contains zero - {2}", x, x.area(), x.contains(&zero_point))
    }
}