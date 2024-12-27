use std::ops::{Add, Mul, Rem, Sub};

pub trait Solution {
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i128,
    pub y: i128,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i128> for Point {
    type Output = Self;

    fn mul(self, other: i128) -> Self::Output {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Rem for Point {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        Point {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }
}

impl Point {
    pub fn new(x: i128, y: i128) -> Self {
        Self { x, y }
    }
}
