use std::ops::{Add, Mul, Rem, Sub};

pub trait Solution {
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vector2d {
    pub x: i64,
    pub y: i64,
}

impl Add for Vector2d {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2d {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vector2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i64> for Vector2d {
    type Output = Self;

    fn mul(self, other: i64) -> Self::Output {
        Vector2d {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Rem for Vector2d {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        Vector2d {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }
}

impl Vector2d {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vector3d {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Add for Vector3d {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3d {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vector3d {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<i64> for Vector3d {
    type Output = Self;

    fn mul(self, other: i64) -> Self::Output {
        Vector3d {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Rem for Vector3d {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        Vector3d {
            x: self.x % other.x,
            y: self.y % other.y,
            z: self.y % other.y,
        }
    }
}

impl Vector3d {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}
