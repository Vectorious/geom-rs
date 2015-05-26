use super::One;
use rect::Rect;
use std::ops::{Add, Sub, Mul};
use std::cmp::{Ord};

pub trait Position2D<T> {
    fn x(&self) -> T;
    fn y(&self) -> T;

    fn x_mut(&mut self) -> &mut T;
    fn y_mut(&mut self) -> &mut T;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point::<T> { x: x, y: y }
    }
}

impl<T> Default for Point<T>
    where T: Default
{
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
    pub fn rect(self, other: Point<T>) -> Rect<T> {
        Rect::from_points(self, other)
    }
}

impl<T> Position2D<T> for Point<T>
    where T: Copy + Clone
{
    fn x(&self) -> T {
        self.x
    }

    fn y(&self) -> T {
        self.y
    }

    fn x_mut(&mut self) -> &mut T {
        &mut self.x
    }

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
