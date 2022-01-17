use std::f64::consts::PI;
use num_traits::Num;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq)]
pub struct Point<T: Num = f64> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rect<T: Num = f64> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Circle<T: Num = f64> {
    pub center: Point<T>,
    pub radius: T,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Figure<T: Num = f64> {
    Circle(Circle<T>),
    Rect(Rect<T>),
}

impl<T: Num> Default for Point<T> {
    fn default() -> Self {
        Self { x: T::zero(), y: T::zero() }
    }
}

impl<T: Num> Default for Rect<T> {
    fn default() -> Self {
        Self {
            x: T::zero(), y: T::zero(),
            width: T::one(), height: T::one()
        }
    }
}

impl<T: Num> Default for Circle<T> {
    fn default() -> Self {
        Self {
            center: Point::default(), radius: T::one()
        }
    }
}

impl<T: Num + PartialOrd + Copy> Rect<T> {
    pub fn contains(&self, point: &Point<T>) -> bool {
        self.x <= point.x && self.y <= point.y && self.x + self.width >= point.x && self.y + self.height >= point.y
    }
}

impl<T: Num + Copy> Rect<T> {
    pub fn area(&self) -> T {
        self.width * self.height
    }
}

fn square_dist<T: Num + Copy>(from: &Point<T>, to: &Point<T>) -> T {
    (from.x - to.x) * (from.x - to.x) + (from.y - to.y) * (from.y - to.y)
}

impl<T: Num + Copy + PartialOrd> Circle<T> {
    pub fn contains(&self, point: &Point<T>) -> bool {
        square_dist(&self.center, point) <= self.radius * self.radius
    }
}

impl<T: Num + Copy + Into<f64>> Circle<T> {
    pub fn area(&self) -> f64 {
        self.radius.into() * self.radius.into() * PI
    }
}

impl<T: Num + Copy + PartialOrd> Figure<T> {
    pub fn contains(&self, point: &Point<T>) -> bool {
        match self {
            Figure::Circle(circle) => circle.contains(point),
            Figure::Rect(rect) => rect.contains(point)
        }
    }
}

impl<T: Num + Copy + Into<f64>> Figure<T> {
    pub fn area(&self) -> f64 {
        match self {
            Figure::Circle(circle) => circle.area(),
            Figure::Rect(rect) => rect.area().into()
        }
    }
}

impl<T: Num + Hash> Hash for Point<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}


impl<T: Num + Hash> Hash for Rect<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.width.hash(state);
        self.height.hash(state);
    }
}


impl<T: Num + Hash> Hash for Circle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.center.hash(state);
        self.radius.hash(state);
    }
}

impl<T: Num + Hash> Hash for Figure<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Figure::Circle(circle) => circle.hash(state),
            Figure::Rect(rect) => rect.hash(state)
        }
    }
} 