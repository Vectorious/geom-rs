pub mod point;
pub mod rect;

pub type Point<T> = point::Point<T>;
pub type Rect<T> = rect::Rect<T>;

trait One {
    fn one() -> Self;
}

impl One for f32 {
    fn one() -> f32 {
        f32::default() + 1.0
    }
}
impl One for f64 {
    fn one() -> f64 {
        f64::default() + 1.0
    }
}
impl One for i16 {
    fn one() -> i16 {
        i16::default() + 1
    }
}
impl One for i32 {
    fn one() -> i32 {
        i32::default() + 1
    }
}
impl One for i64 {
    fn one() -> i64 {
        i64::default() + 1
    }
}
impl One for i8 {
    fn one() -> i8 {
        i8::default() + 1
    }
}
impl One for isize {
    fn one() -> isize {
        isize::default() + 1
    }
}
impl One for u16 {
    fn one() -> u16 {
        u16::default() + 1
    }
}
impl One for u32 {
    fn one() -> u32 {
        u32::default() + 1
    }
}
impl One for u64 {
    fn one() -> u64 {
        u64::default() + 1
    }
}
impl One for u8 {
    fn one() -> u8 {
        u8::default() + 1
    }
}
impl One for usize {
    fn one() -> usize {
        usize::default() + 1
    }
}
