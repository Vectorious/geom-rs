use Point;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Rect {
    top_left: Point,
    bottom_right: Point
}

impl Rect {
    pub fn new(left: i32, top: i32, width: i32, height: i32) -> Rect {
        Rect {
            bottom_right: Point{ x: left + width - 1, y: top + height - 1 },
            top_left: Point { x: left, y: top }
        }
    }

    pub fn from_points(top_left: Point, bottom_right: Point) -> Rect {
        Rect { top_left: top_left, bottom_right: bottom_right }
    }

    pub fn width(&self) -> i32 {
        self.right() - self.left() + 1
    }

    pub fn height(&self) -> i32 {
        self.bottom() - self.top() + 1
    }

    pub fn top_left(&self) -> Point {
        self.top_left
    }

    pub fn top(&self) -> i32 {
        self.top_left.y
    }

    pub fn left(&self) -> i32 {
        self.top_left.x
    }

    pub fn bottom_right(&self) -> Point {
        self.bottom_right
    }

    pub fn bottom(&self) -> i32 {
        self.bottom_right.y
    }

    pub fn right(&self) -> i32 {
        self.bottom_right.x
    }

    pub fn area(&self) -> i32 {
        self.width() * self.height()
    }

    pub fn intersect(&self, other: &Rect) -> Option<Rect> {
        use std::cmp::{min, max};

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

    pub fn split_column_mut(&mut self, col: i32) -> Rect {
        let new_rect = Rect::from_points(Point::new(col, self.top()), self.bottom_right());
        self.bottom_right.x = col - 1;
    
        new_rect
    }

    pub fn split_row_mut(&mut self, row: i32) -> Rect {
        let new_rect = Rect::from_points(Point::new(self.left(), row), self.bottom_right());
        self.bottom_right.y = row - 1;

        new_rect
    }

    pub fn iter(&self) -> Iter {
        self.into_iter()
    }

    pub fn columns(&self) -> Vec<Rect> {
        let mut columns: Vec<Rect> = Vec::new();

        for x in (self.left()..self.right()+1) {
            columns.push(Rect::new(x, self.top(), 1, self.height() + 1));
        }

        columns
    }

    pub fn rows(&self) -> Vec<Rect> {
        let mut rows: Vec<Rect> = Vec::new();

        for y in (self.top()..self.bottom()+1) {
            rows.push(Rect::new(self.left(), y, self.width() + 1, 1));
        }

        rows
    }
}

impl IntoIterator for Rect {
    type Item = Point;
    type IntoIter = Iter;
    fn into_iter(self) -> Self::IntoIter {
        Iter { cur: self.top_left, rect: self }
    }
}

pub struct Iter {
    rect: Rect,
    cur: Point
}

impl Iterator for Iter {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        let point = self.cur;

        if self.cur.x < self.rect.right() {
            self.cur.x += 1;
        } else if self.cur.y <= self.rect.bottom() {
            self.cur.x = self.rect.left();
            self.cur.y += 1;
        }

        if point.x <= self.rect.right() && point.y <= self.rect.bottom() {
            Some(point)
        } else {
            None
        }
    }
}
