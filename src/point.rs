use rect::Rect;
use std::ops::{Add, Sub};

trait Point<T> {
    fn new(x: T, y: T) -> Point<T>;
    fn x() -> T;
    fn y() -> T;
}
