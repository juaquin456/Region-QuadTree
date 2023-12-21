use std::ops::{Add, Div, Sub};

use serde::{Deserialize, Serialize};

trait Contains<T> {
    /// Return true if the object is contained in the region.
    fn contains(&self, obj: T) -> bool;
}

#[derive(PartialEq, PartialOrd, Copy, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Point {
            x: value.0,
            y: value.1,
        }
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

impl Div<u32> for Point {
    type Output = Point;
    fn div(self, rhs: u32) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

/// A bounding box is a rectangle that is defined by its bottom-left corner and its width and height.
#[derive(Serialize, Deserialize)]
pub struct BoundingBox {
    min: Point,
    max: Point,
}

impl BoundingBox {
    pub(crate) fn get_bounds(&self) -> [[Point; 2]; 4] {
        [
            [self.min, Point::from((self.min.x, self.max.y))],
            [Point::from((self.min.x, self.max.y)), self.max],
            [self.max, Point::from((self.max.x, self.min.y))],
            [Point::from((self.max.x, self.min.y)), self.min],
        ]
    }
}

impl BoundingBox {
    pub fn new(min: Point, max: Point) -> Self {
        assert!(min <= max);

        BoundingBox { min, max }
    }

    pub fn min(&self) -> &Point {
        &self.min
    }

    pub fn max(&self) -> &Point {
        &self.max
    }

    /// Return the center of the bounding box.
    pub fn center(&self) -> Point {
        (self.min + self.max) / 2
    }
}

impl Contains<Point> for BoundingBox {
    fn contains(&self, p: Point) -> bool {
        (self.min.x <= p.x) && (p.x <= self.max.x) && (self.min.y <= p.y) && (p.y <= self.max.y)
    }
}

impl Contains<BoundingBox> for BoundingBox {
    fn contains(&self, b: BoundingBox) -> bool {
        self.contains(b.min) && self.contains(b.max)
    }
}
