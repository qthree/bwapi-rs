use typenum::{Integer, P1, P8, P32};
use std::marker::PhantomData;

use std::cmp::PartialEq;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Rem;

#[derive(Default)]
pub struct Point<T, S: Integer> {
    x: T,
    y: T,
    scale: PhantomData<S>
}

impl<T, S> Point<T, S> where S : Integer {
    fn new(x: T, y: T) -> Point<T, S> {
        Point::<T, S> { x: x, y: y, scale: PhantomData}
    }
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
    fn get_scale() -> i32 {
        S::to_i32()
    }
}

pub type Position = Point<i32, P1>;
pub type WalkPosition = Point<i32, P8>;
pub type TilePosition = Point<i32, P32>;

// Operators

impl<T, S> PartialEq for Point<T, S>
        where T: Eq, S: Integer {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

impl<T, S> Add for Point<T, S>
        where T: Add<Output=T>, S: Integer {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl<T, S> Sub for Point<T, S>
        where T: Sub<Output=T>, S: Integer {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl<T, S> Mul<T> for Point<T, S>
        where T: Mul<Output=T> + Copy, S: Integer {
    type Output = Self;
    fn mul(self, other: T) -> Self {
        Self::new(self.x * other, self.y * other)
    }
}

impl<T, S> Div<T> for Point<T, S>
        where T: Div<Output=T> + Copy + PartialOrd<usize>, S: Integer {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        if rhs == 0 {
            //let magic_coord: T = 32000 / self.get_scale();
            //Self::new(magic_coord, magic_coord)
            Self::new(self.x, self.y)
        } else {
            Self::new(self.x / rhs, self.y / rhs)
        }

    }
}


impl<T, S> Rem<T> for Point<T, S>
        where T: Rem<Output=T> + Div<Output=T> + Copy + PartialOrd<usize>, S: Integer {
    type Output = Self;
    fn rem(self, modulus: T) -> Self {
        if modulus == 0 {
            //let magic_coord: T = 32000 / self.get_scale();
            //Self::new(magic_coord, magic_coord)
            Self::new(self.x, self.y)
        } else {
            Self::new(self.x % modulus, self.y % modulus)
        }

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
        assert!(Position::new(8, 16) == Position::from( WalkPosition::new(1, 2) ));
    }
}
