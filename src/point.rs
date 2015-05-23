use rect::Rect;
use std::ops::{Add, Sub};

pub const ZERO: Point = Point { x: 0, y: 0 };
pub const UP: Point = Point { x: 0, y: -1 };
pub const DOWN: Point = Point { x: 0, y: 1 };
pub const LEFT: Point = Point { x: -1, y: 0 };
pub const RIGHT: Point = Point { x: 1, y: 0 };


/// A simple two-dimensional Point structure.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    /// Creates a new point with the given `x` and `y` coordinates.
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    /// Creates a rect with `self` as the top-left point and `other` as the bottom-right point.
    pub fn rect(&self, other: Point) -> Rect {
        Rect::from_points(*self, other)
    }
}


impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

/// Adds an `i32` value to both the `x` and `y` values of a point.
impl Add<i32> for Point {
    type Output = Point;
    fn add(self, other: i32) -> Point {
        Point { x: self.x + other, y: self.y + other }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point { x: self.x - other.x, y: self.y - other.y }
    }
}

/// Subtracts an `i32` value from both the `x` and `y` values of a point.
impl Sub<i32> for Point {
    type Output = Point;
    fn sub(self, other: i32) -> Point {
        Point { x: self.x - other, y: self.y - other }
    }
}

