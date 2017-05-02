use num::traits::{Num, NumCast, ToPrimitive};
use typenum::{Integer, P1, P8, P32};
use std::marker::PhantomData;

use std::cmp::PartialEq;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Rem;

#[derive(Default)]
pub struct Point<T, S> {
    x: T,
    y: T,
    scale: PhantomData<S>
}

impl<T, S> Point<T, S> {
    fn new(x: T, y: T) -> Self {
        Point::<T, S> { x: x, y: y, scale: PhantomData}
    }
}

impl<T: Num + NumCast + Copy, S> Point<T, S> {
    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).to_f64().unwrap().sqrt()
    }
    /*
    fn distance(&self, to: &Self) -> f64 {
        (self - to).length()
    }*/
}

impl<T: NumCast, S: Integer> Point<T, S> {
    fn get_scale() -> T {
         NumCast::from(S::to_isize()).unwrap()
    }
}

impl<T: Num + NumCast + Ord + Copy, S: Integer> Point<T, S> {
    fn from<F>(other: Point<T, F>) -> Self
            where F: Integer, T: Mul<i32,Output=T> + Div<i32,Output=T> {
        let self_scale = Self::get_scale();
        let other_scale = Point::<T, F>::get_scale();
        if other_scale > self_scale {
            let scalar = other_scale / self_scale;
            Self::new(other.x * scalar, other.y * scalar)
        } else {
            let scalar = self_scale / other_scale;
            Self::new(other.x / scalar, other.y / scalar)
        }
    }
}

pub type Position = Point<i32, P1>;
pub type WalkPosition = Point<i32, P8>;
pub type TilePosition = Point<i32, P32>;

// Operators

impl<T: PartialEq, S> PartialEq for Point<T, S> {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

impl<T: Num, S> Add for Point<T, S> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: Num, S> Sub for Point<T, S> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl<T: Num + Copy, S> Mul<T> for Point<T, S> {
    type Output = Self;
    fn mul(self, scalar: T) -> Self {
        Self::new(self.x * scalar, self.y * scalar)
    }
}

impl<T: Num + Copy, S> Div<T> for Point<T, S> {
    type Output = Self;
    fn div(self, scalar: T) -> Self {
        Self::new(self.x / scalar, self.y / scalar)
    }
}

impl<T: Num + Copy, S> Rem<T> for Point<T, S> {
    type Output = Self;
    fn rem(self, scalar: T) -> Self {
        Self::new(self.x % scalar, self.y % scalar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;
    #[test]
    fn test_position() {
        assert!(8 == size_of::<Position>());
        assert!(Position {x: 0, y: 0, scale: PhantomData} == Position::default());
        assert!(Position {x: 24, y: 42, scale: PhantomData} == Position::new(24, 42));
        assert!(Position::new(3, 13) == Position::new(1, 5) + Position::new(2, 8));
        assert!(Position::new(1, 5) == Position::new(3, 13) - Position::new(2, 8));
        assert!(Position::new(2, 10) == Position::new(1, 5) * 2);
        assert!(Position::new(12, 21) == Position::new(24, 42) / 2);
        assert!(Position::new(2, 1) == Position::new(2, 7) % 3);
        assert!(Position::new(8, 16) == Position::from( WalkPosition::new(1, 2) ));
    }
    #[test]
    #[should_panic]
    fn test_position_div_zero() {
        Position::default() / 0;
    }
    #[test]
    #[should_panic]
    fn test_position_rem_zero() {
        Position::default() % 0;
    }
}
