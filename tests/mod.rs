extern crate geom;
use geom::*;

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
    let r = Point::default().rect(Point::new(4, 9));
    let s = Rect::from_points(Point::default(), Point::new(4, 9));

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
    assert_eq!(r.bottom(), 4);
    assert_eq!(r.right(), 9);
}

#[test]
fn rect_iter() {
    let mut r = Rect::new(0, 0, 10, 15).iter();

    assert_eq!(r.nth(0), Some(Point::new(0, 0)));  // [0]
    assert_eq!(r.nth(8), Some(Point::new(9, 0)));  // [9]
    assert_eq!(r.nth(0), Some(Point::new(0, 1)));  // [10]
    assert_eq!(r.last(), Some(Point::new(9, 14)));
}

#[test]
fn rect_contains() {
    let r = Rect::new(2, 3, 10, 15);

    assert!(r.contains(Point::new(2, 3)));
    assert!(r.contains(Point::new(5, 5)));
    assert!(r.contains(Point::new(11, 17)));

    assert_eq!(r.contains(Point::new(0, 0)), false);
    assert_eq!(r.contains(Point::new(5, 0)), false);
    assert_eq!(r.contains(Point::new(12, 17)), false);
}

#[test]
fn rect_rows() {
    let r = Rect::new(4, 2, 12, 14);

    for row in r.rows() {
        for point in row {
            assert!(r.contains(point), "{:?} not in Rect {:?}", point, r);
        }
    }
}

#[test]
fn rect_columns() {
    let r = Rect::new(4, 2, 12, 14);

    for row in r.columns() {
        for point in row {
            assert!(r.contains(point), "{:?} not in Rect {:?}", point, r);
        }
    }
}

#[test]
fn point_operators() {
    let a = Point::new(3, 6);
    let b = Point::new(7, 4);

    assert_eq!(a - a, Point::default());

    assert_eq!(a + b, Point::new(10, 10));
    assert_eq!(a - b, Point::new(-4, 2));

    assert_eq!(a + 7, Point::new(10, 13));
    assert_eq!(a - 3, Point::new(0, 3));

    let f_a = Point::new(4.14, 4.50);
    let f_b = Point::new(1.42, 0.11);

    assert_eq!(f_a - f_a, Point::default());

    assert_eq!(f_a + f_b, Point::new(5.56, 4.61));
    assert_eq!(f_a + 1.0, Point::new(5.14, 5.50));
    assert!(f_a - 1.0 != Point::new(3.14, 3.51));
}
