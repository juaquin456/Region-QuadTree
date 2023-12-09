use std::ops::{Add, Sub};

pub struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn l1(p1: Self, p2: Self) -> u32 {
        (p2.x - p1.x + p2.y - p1.y).abs()
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// A bounding box is a rectangle that is defined by its bottom-left corner and its width and height.
pub struct BoundingBox {
    min: Point,
    max: Point,
}

impl BoundingBox {
    fn new(min: Point, max: Point) -> Self {
        BoundingBox{
            min,
            max,
        }
    }

    fn center(self) -> Point {
        (self.min + self.max) / 2
    }

}