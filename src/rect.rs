use super::One;
use point::{Point, Position2D};
use std::ops::{Add, Sub, Mul};
use std::cmp::{Ord, min, max};
use std::iter::Iterator;

/// A generic rectangle structure.
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct Rect<T> {
    top_left: Point<T>,
    bottom_right: Point<T>
}

impl<T> Rect<T>
    where T: Add<T, Output=T> +
             Sub<T, Output=T> +
             Mul<T, Output=T> +
             Ord + One + Default + Copy + Clone
{
    /// Returns a new rectangle with the supplied position and dimensions.
    pub fn new(left: T, top: T, width: T, height: T) -> Rect<T> {
        Rect::<T> {
            bottom_right: Point::<T>::new(left + width - T::one(), top + height - T::one()),
            top_left: Point::<T>::new(left, top)
        }
    }

    /// Returns a new rectangle with the given top-left and bottom-right points.
    pub fn from_points(top_left: Point<T>, bottom_right: Point<T>) -> Rect<T> {
        Rect { top_left: top_left, bottom_right: bottom_right }
    }

    /// Returns the width of the rectangle.
    pub fn width(&self) -> T {
        self.right() - self.left() + T::one()
    }

    /// Returns the height of the rectangle.
    pub fn height(&self) -> T {
        self.bottom() - self.top() + T::one()
    }

    /// Returns a copy of the top-left point of the rectangle.
    pub fn top_left(&self) -> Point<T> {
        self.top_left
    }

    /// Returns the `y` coordinate of the top side of the rectangle.
    pub fn top(&self) -> T {
        self.top_left.y()
    }

    /// Returns the `x` coordinate of the left side of the rectangle.
    pub fn left(&self) -> T {
        self.top_left.x()
    }

    /// Returns a copy of the bottom-right point of the rectangle.
    pub fn bottom_right(&self) -> Point<T> {
        self.bottom_right
    }

    /// Returns the `y` coordinate of the bottom side of the rectangle.
    pub fn bottom(&self) -> T {
        self.bottom_right.y()
    }

    /// Returns the `x` coordinate of the right side of the rectangle.
    pub fn right(&self) -> T {
        self.bottom_right.x()
    }

    /// Returns the area of the rectangle.
    pub fn area(&self) -> T {
        self.width() * self.height()
    }

    /// Returns `true` if the given point lies within the bounds of the rectangle,
    /// and `false` otherwise.
    pub fn contains(&self, point: Point<T>) -> bool {
        self.left() <= point.x() && point.x() <= self.right() &&
        self.top() <= point.y() && point.y() <= self.bottom()
    }

    /// If `self` and `other` intersect, then the intersection of the two rectangles
    /// is returned as a new rectangle, otherwise `None` is returned.
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

    /// Splits the rectangle at the given column `col`. The right side of the left part
    /// of the resulting split will be at `col - 1`, and the left side of the right part
    /// will be at `col`. The current rectangle will be modified in-place to be the left
    /// part, and the right part will be returned as a new rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use geom::Rect;
    ///
    /// let mut r = Rect::new(0, 0, 10, 10);
    /// let s = r.split_column_mut(5);
    ///
    /// assert_eq!(r.top(), 0);
    /// assert_eq!(r.left(), 0);
    /// assert_eq!(r.bottom(), 9);
    /// assert_eq!(r.right(), 4);
    ///
    /// assert_eq!(s.top(), 0);
    /// assert_eq!(s.left(), 5);
    /// assert_eq!(s.bottom(), 9);
    /// assert_eq!(s.right(), 9);
    /// ```
    pub fn split_column_mut(&mut self, col: T) -> Rect<T> {
        let new_rect = Rect::from_points(Point::new(col, self.top()), self.bottom_right());
        *self.bottom_right.x_mut() = col - T::one();

        new_rect
    }

    /// Splits the rectangle at the given row `row`. The bottom side of the top part
    /// of the resulting split will be at `row - 1`, and the top side of the bottom part
    /// will be at `row`. The current rectangle will be modified in-place to be the top
    /// part, and the bottom part will be returned as a new rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use geom::Rect;
    ///
    /// let mut r = Rect::new(0, 0, 10, 10);
    /// let s = r.split_row_mut(5);
    ///
    /// assert_eq!(r.top(), 0);
    /// assert_eq!(r.left(), 0);
    /// assert_eq!(r.bottom(), 4);
    /// assert_eq!(r.right(), 9);
    ///
    /// assert_eq!(s.top(), 5);
    /// assert_eq!(s.left(), 0);
    /// assert_eq!(s.bottom(), 9);
    /// assert_eq!(s.right(), 9);
    /// ```
    pub fn split_row_mut(&mut self, row: T) -> Rect<T> {
        let new_rect = Rect::from_points(Point::new(self.left(), row), self.bottom_right());
        *self.bottom_right.y_mut() = row - T::one();

        new_rect
    }

    /// Returns a `Vec` containing a one-width rectangle for each column of the rectangle.
    pub fn columns(&self) -> Vec<Rect<T>> {
        let mut columns: Vec<Rect<T>> = Vec::new();

        let mut x = self.left();
        while x <= self.right() {
            columns.push(Rect::new(x, self.top(), T::one(), self.height()));
            x = x + T::one();
        }

        columns
    }

    /// Returns a `Vec` containing a one-height rectangle for each row of the rectangle.
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
    where T: Add<T, Output=T> +
             Sub<T, Output=T> +
             Mul<T, Output=T> +
             Ord + One + Default + Copy + Clone
{
    /// Returns an iterator over each point in the rectangle, going from left-to-right and
    /// top-to-bottom.
    pub fn iter(&self) -> Iter<T> {
        self.into_iter()
    }
}

impl<T> IntoIterator for Rect<T>
    where T: Add<T, Output=T> +
             Sub<T, Output=T> +
             Mul<T, Output=T> +
             Ord + One + Default + Copy + Clone
{
    type Item = Point<T>;
    type IntoIter = Iter<T>;
    fn into_iter(self) -> Iter<T> {
        Iter::<T> { cur: self.top_left(), rect: self }
    }
}

/// An iterator over a rectangle, returning each point within the rectangle going from
/// left-to-right and top-to-bottom.
pub struct Iter<T> {
    rect: Rect<T>,
    cur: Point<T>
}

impl<T> Iterator for Iter<T>
    where T: Add<T, Output=T> +
             Sub<T, Output=T> +
             Mul<T, Output=T> +
             Ord + One + Default + Copy + Clone
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let cur = self.cur;
        let rect = self.rect;

        // Calculate the exact amount of points remaining in the iterator.
        // We do this by getting the area of the rect, subtracting all of the points from rows
        // that have already been passed, and then subtracting the columns that have been passed
        // in the current row.
        let rem = rect.area() -                              // area
                  ((cur.y() - rect.top()) * rect.width()) -  // rows we've passed
                  (cur.x() - rect.left());                   // columns we've passed in the row

        // Convert rem to usize using a very hacky and innefficient method.
        // We count the number of iterations it takes to get from T::default() to rem, by adding
        // T::one(). Hopefully this can be replaced by a more efficient method at some point.
        let mut size: usize = 0;
        let mut count: T = T::default();
        while count < rem {
            count = count + T::one();
            size += 1;
        }

        (size, Some(size))
    }
}

impl<T> DoubleEndedIterator for Iter<T>
    where T: Add<T, Output=T> +
             Sub<T, Output=T> +
             Mul<T, Output=T> +
             Ord + One + Default + Copy + Clone
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let point = self.cur;

        if self.cur.x() > self.rect.left() {
            *self.cur.x_mut() = self.cur.x() - T::one();
        } else if self.cur.y() >= self.rect.top() {
            *self.cur.x_mut() = self.rect.right();
            *self.cur.y_mut() = self.cur.y() - T::one();
        }

        if point.x() >= self.rect.left() && point.y() >= self.rect.top() {
            Some(point)
        } else {
            None
        }
    }
}

impl<T> ExactSizeIterator for Iter<T>
    where T: Add<T, Output=T> +
             Sub<T, Output=T> +
             Mul<T, Output=T> +
             Ord + One + Default + Copy + Clone
{  }
