use std::ops::Add;

pub mod point;
pub mod rect;

pub type Point<T> = point::Point<T>;
pub type Rect<T> = rect::Rect<T>;

trait One<T>
    where T: Add<i32, Output=T> +
             Default
{
    fn one() -> T {
        T::default() + 1
    }
}

impl<T> One<T> for T
    where T: Default + Add<i32, Output=T>
{ /* derived implementor */ }
