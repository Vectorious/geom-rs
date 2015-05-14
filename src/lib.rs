pub mod point;
pub mod rect;

pub type Point = point::Point;
pub type Rect = rect::Rect;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_split_row_mut() {
        let mut r = Rect::new(0, 0, 10, 10);
        let s = r.split_row_mut(5);

        assert_eq!(r.top(), 0);
        assert_eq!(r.left(), 0);
        assert_eq!(r.bottom(), 4);
        assert_eq!(r.right(), 9);

        assert_eq!(s.top(), 5);
        assert_eq!(s.left(), 0);
        assert_eq!(s.bottom(), 9);
        assert_eq!(s.right(), 9);
    }

    #[test]
    fn rect_split_column_mut() {
        let mut r = Rect::new(0, 0, 10, 10);
        let s = r.split_column_mut(5);

        assert_eq!(r.top(), 0);
        assert_eq!(r.left(), 0);
        assert_eq!(r.bottom(), 9);
        assert_eq!(r.right(), 4);

        assert_eq!(s.top(), 0);
        assert_eq!(s.left(), 5);
        assert_eq!(s.bottom(), 9);
        assert_eq!(s.right(), 9);
    }

    #[test]
    fn rect_from_points() {
        let r = point::ZERO.rect(Point::new(4, 9));
        let s = Rect::from_points(point::ZERO, Point::new(4, 9));

        assert_eq!(r, s);

        assert_eq!(r.top(), 0);
        assert_eq!(r.left(), 0);
        assert_eq!(r.bottom(), 9);
        assert_eq!(r.right(), 4);
    }

    #[test]
    fn rect_from_new() {
        let r = Rect::new(0, 0, 10, 5);

        assert_eq!(r.top(), 0);
        assert_eq!(r.left(), 0);
        assert_eq!(r.bottom(), 9);
        assert_eq!(r.right(), 4);
    }


    #[test]
    fn point_operators() {
        let a = Point::new(3, 6);
        let b = Point::new(7, 4);

        assert_eq!(a - a, point::ZERO);

        assert_eq!(a + b, Point::new(10, 10));
        assert_eq!(a - b, Point::new(-4, 2));

        assert_eq!(a + 7, Point::new(10, 13));
        assert_eq!(a - 3, Point::new(0, 3));
    }
}
