use super::One;
use rect::Rect;
use std::ops::{Add, Sub, Mul};
use std::cmp::{Ord};

/// A type with `x` and `y` coordinates.
pub trait Position2D<T> {
    /// Returns a copy of the `x` coordinate of the position.
    fn x(&self) -> T;

    /// Returns a copy of the `y` coordinate of the position.
    fn y(&self) -> T;

    /// Returns a mutable reference to the `x` coordinate of the position.
    fn x_mut(&mut self) -> &mut T;

    /// Returns a mutable reference to the `y` coordinate of the position.
    fn y_mut(&mut self) -> &mut T;
}

/// A generic two-dimensional point structure.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    /// Returns a new `Point` with the given `x` and `y` coordinates.
    pub fn new(x: T, y: T) -> Point<T> {
        Point::<T> { x: x, y: y }
    }
}

impl<T> Default for Point<T>
    where T: Default
{
    /// Returns a new `Point` with the `x` and `y` coordinates being that of the default value of `T`.
    fn default() -> Point<T> {
        Point::<T>::new(T::default(), T::default())
    }
}

impl<T> Point<T>
    where T: Add<i32, Output=T> +
             Add<T, Output=T> +
             Sub<T, Output=T> +
             Mul<T, Output=T> +
             Ord + One<T> + Default + Copy + Clone
{
    /// Returns a new `Rect` with the top-left point being the value of `self`
    /// and the bottom-right point being the value of `other`.
    pub fn rect(self, other: Point<T>) -> Rect<T> {
        Rect::from_points(self, other)
    }
}

impl<T> Position2D<T> for Point<T>
    where T: Copy + Clone
{
    /// Returns a copy of the `x` coordinate of the point.
    fn x(&self) -> T {
        self.x
    }

    /// Returns a copy of the `y` coordinate of the point.
    fn y(&self) -> T {
        self.y
    }

    /// Returns a mutable reference to the `x` coordinate of the point.
    fn x_mut(&mut self) -> &mut T {
        &mut self.x
    }

    /// Returns a mutable reference to the `x` coordinate of the point.
    fn y_mut(&mut self) -> &mut T {
        &mut self.y
    }
}

impl<T> Add<Point<T>> for Point<T>
    where Point<T>: Position2D<T>,
          T: Add<T, Output=T>
{
    type Output = Point<T>;
    fn add(self, other: Point<T>) -> Point<T> {
        Point::<T>::new(self.x() + other.x(), self.y() + other.y())
    }
}

impl<T> Add<T> for Point<T>
    where Point<T>: Position2D<T>,
          T: Add<T, Output=T> + Clone + Copy
{
    type Output = Point<T>;
    fn add(self, other: T) -> Point<T> {
        Point::<T>::new(self.x() + other, self.y() + other)
    }
}

impl<T> Sub<Point<T>> for Point<T>
    where Point<T>: Position2D<T>,
          T: Sub<T, Output=T>
{
    type Output = Point<T>;
    fn sub(self, other: Point<T>) -> Point<T> {
        Point::<T>::new(self.x() - other.x(), self.y() - other.y())
    }
}

impl<T> Sub<T> for Point<T>
    where Point<T>: Position2D<T>,
          T: Sub<T, Output=T> + Clone + Copy
{
    type Output = Point<T>;
    fn sub(self, other: T) -> Point<T> {
        Point::<T>::new(self.x() - other, self.y() - other)
    }
}
