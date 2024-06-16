use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Position {
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn new<T>(x: T, y: T, z: T) -> Self
    where
        f32: std::convert::From<T>,
    {
        Self {
            x: f32::from(x),
            y: f32::from(y),
            z: f32::from(z),
        }
    }
}

impl Add<&Position> for Position {
    type Output = Position;
    fn add(self, rhs: &Position) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Add<Position> for Position {
    type Output = Position;
    fn add(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Mul for &Position {
    type Output = Position;
    fn mul(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl Mul<u32> for &Position {
    type Output = Position;
    fn mul(self, rhs: u32) -> Self::Output {
        Position {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
            z: self.z * rhs as f32,
        }
    }
}
impl Mul<f32> for Position {
    type Output = Position;
    fn mul(self, rhs: f32) -> Self::Output {
        Position {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Mul<u32> for Position {
    type Output = Position;
    fn mul(self, rhs: u32) -> Self::Output {
        Position {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
            z: self.z * rhs as f32,
        }
    }
}
impl Div<u32> for Position {
    type Output = Self;
    fn div(mut self, rhs: u32) -> Self::Output {
        self.x /= rhs as f32;
        self.y /= rhs as f32;
        self.z /= rhs as f32;
        self
    }
}
impl Sub for Position {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}
