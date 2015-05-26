use super::One;
use point::{Point, Position2D};
use std::ops::{Add, Sub, Mul};
use std::cmp::{Ord, min, max};
use std::iter::Iterator;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Rect<T> {
    top_left: Point<T>,
    bottom_right: Point<T>
}

impl<T> Rect<T>
    where T: Add<i32, Output=T> +
             Add<T, Output=T> +
             Sub<T, Output=T> +
             Mul<T, Output=T> +
             Ord + One<T> + Default + Copy + Clone
{
    pub fn new(left: T, top: T, width: T, height: T) -> Rect<T> {
        Rect::<T> {
            bottom_right: Point::<T>::new(left + width - T::one(), top + height - T::one()),
            top_left: Point::<T>::new(left, top)
        }
    }

    pub fn from_points(top_left: Point<T>, bottom_right: Point<T>) -> Rect<T> {
        Rect { top_left: top_left, bottom_right: bottom_right }
    }

    pub fn width(&self) -> T {
        self.right() - self.left() + T::one()
    }

    pub fn height(&self) -> T {
        self.bottom() - self.top() + T::one()
    }

    pub fn top_left(&self) -> Point<T> {
        self.top_left
    }

    pub fn top(&self) -> T {
        self.top_left.y()
    }

    pub fn left(&self) -> T {
        self.top_left.x()
    }

    pub fn bottom_right(&self) -> Point<T> {
        self.bottom_right
    }

    pub fn bottom(&self) -> T {
        self.bottom_right.y()
    }

    pub fn right(&self) -> T {
        self.bottom_right.x()
    }

    pub fn area(&self) -> T {
        self.width() * self.height()
    }

    pub fn contains(&self, point: Point<T>) -> bool {
        self.left() <= point.x() && point.x() <= self.right() &&
        self.top() <= point.y() && point.y() <= self.bottom()
    }

    pub fn intersect(&self, other: &Rect<T>) -> Option<Rect<T>> {
        let left = max(self.left(), other.left());
        let right = min(self.right(), other.right());
        let top = max(self.top(), other.top());
        let bottom = min(self.bottom(), other.bottom());

        if left <= right && top <= bottom {
            Some(Point::new(left, top).rect(Point::new(right, bottom)))
        } else {
            None
        }
    }

    pub fn split_column_mut(&mut self, col: T) -> Rect<T> {
        let new_rect = Rect::from_points(Point::new(col, self.top()), self.bottom_right());
        *self.bottom_right.x_mut() = col - T::one();
    
        new_rect
    }

    pub fn split_row_mut(&mut self, row: T) -> Rect<T> {
        let new_rect = Rect::from_points(Point::new(self.left(), row), self.bottom_right());
        *self.bottom_right.y_mut() = row - T::one();

        new_rect
    }

    pub fn columns(&self) -> Vec<Rect<T>> {
        let mut columns: Vec<Rect<T>> = Vec::new();

        let mut x = self.left();
        while x <= self.right() {
            columns.push(Rect::new(x, self.top(), T::one(), self.height()));
            x = x + T::one();
        }

        columns
    }

    pub fn rows(&self) -> Vec<Rect<T>> {
        let mut rows: Vec<Rect<T>> = Vec::new();

        let mut y = self.top();
        while y <= self.bottom() {
            rows.push(Rect::new(self.left(), y, self.width(), T::one()));
            y = y + T::one();
        }

        rows
    }
}

impl<T> Rect<T>
    where T: Add<i32, Output=T> +
             Add<T, Output=T> +
             Sub<T, Output=T> +
             Mul<T, Output=T> +
             Ord + One<T> + Default + Copy + Clone
{

    pub fn iter(&self) -> Iter<T> {
        self.into_iter()
    }
}

impl<T> IntoIterator for Rect<T>
    where T: Add<i32, Output=T> +
             Add<T, Output=T> +
             Sub<T, Output=T> +
             Mul<T, Output=T> +
             Ord + One<T> + Default + Copy + Clone
{
    type Item = Point<T>;
    type IntoIter = Iter<T>;
    fn into_iter(self) -> Iter<T> {
        Iter::<T> { cur: self.top_left(), rect: self }
    }
}

pub struct Iter<T> {
    rect: Rect<T>,
    cur: Point<T>
}

impl<T> Iterator for Iter<T>
    where T: Add<i32, Output=T> +
             Add<T, Output=T> +
             Sub<T, Output=T> +
             Mul<T, Output=T> +
             Ord + One<T> + Default + Copy + Clone
{
    type Item = Point<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let point = self.cur;

        if self.cur.x() < self.rect.right() {
            *self.cur.x_mut() = self.cur.x() + T::one();
        } else if self.cur.y() <= self.rect.bottom() {
            *self.cur.x_mut() = self.rect.left();
            *self.cur.y_mut() = self.cur.y() + T::one();
        }

        if point.x() <= self.rect.right() && point.y() <= self.rect.bottom() {
            Some(point)
        } else {
            None
        }
    }
}
