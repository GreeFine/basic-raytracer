use std::ops::{Add, Mul};

#[derive(Debug)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    pub fn from_u32(value: u32) -> Self {
        Rgb {
            red: ((value & 0xFF0000) >> 16) as u8,
            green: ((value & 0x00FF00) >> 8) as u8,
            blue: (value & 0x0000FF) as u8,
        }
    }
    pub fn from_percent(red: f32, green: f32, blue: f32) -> Self {
        assert!(red.clamp(-1.0, 1.0) == red);
        assert!(green.clamp(-1.0, 1.0) == green);
        assert!(blue.clamp(-1.0, 1.0) == blue);
        Rgb {
            red: (255.0 * red) as u8,
            green: (255.0 * green) as u8,
            blue: (255.0 * blue) as u8,
        }
    }
    pub fn to_ppm(&self) -> String {
        format!("{} {} {} ", self.red, self.green, self.blue)
    }
}

impl Mul<f32> for Rgb {
    type Output = Rgb;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            red: (self.red as f32 * rhs) as u8,
            green: (self.green as f32 * rhs) as u8,
            blue: (self.blue as f32 * rhs) as u8,
        }
    }
}
impl Add for Rgb {
    type Output = Rgb;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}
