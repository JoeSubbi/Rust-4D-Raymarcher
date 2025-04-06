use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Default)]
pub struct Float2
{
    x : f32,
    y : f32,
}

impl Float2
{
    pub fn new(x: f32, y: f32) -> Float2
    {
        return Float2{ x: x, y: y};
    }

    pub fn length(&self) -> f32
    {
        let length : f32 = self.length_squared();
        return f32::sqrt(length);
    }

    pub fn length_squared(&self) -> f32
    {
        return self.x * self.x + self.y * self.y;
    }

    pub fn normalize(&mut self)
    {
        let length : f32 = self.length();
        if length > 0.0
        {
            *self = *self / self.length();
        }
    }

    pub fn normalized(&self) -> Float2
    {
        let length : f32 = self.length();
        if length > 0.0
        {
            return *self / self.length();
        }
        else {
            return *self;
        }
    }
}

// Output formatting
impl Display for Float2 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

// Float2 + Float2
impl Add for Float2 {
    type Output = Float2;
 
    fn add(self, v: Float2) -> Float2 {
        return Float2::new(self.x + v.x, self.y + v.y);
    }
}

// Float2 + f32
impl Add<f32> for Float2 {
    type Output = Float2;
 
    fn add(self, v: f32) -> Float2 {
        return Float2::new(self.x + v, self.y + v);
    }
}

// Float2 += Float2
impl AddAssign for Float2 {
    fn add_assign(&mut self, v: Float2) {
        *self = *self + v;
    }
}

// Float2 += f32
impl AddAssign<f32> for Float2 {
    fn add_assign(&mut self, v: f32) {
        *self = *self + v;
    }
}
 
// Float2 - Float2
impl Sub for Float2 {
    type Output = Float2;
 
    fn sub(self, v: Float2) -> Float2 {
        return Float2::new(self.x - v.x, self.y - v.y);
    }
}

// Float2 - f32
impl Sub<f32> for Float2 {
    type Output = Float2;
 
    fn sub(self, v: f32) -> Float2 {
        return Float2::new(self.x - v, self.y - v);
    }
}

// Float2 -= Float2
impl SubAssign for Float2 {
    fn sub_assign(&mut self, v: Float2) {
        *self = *self - v;
    }
}

// Float2 -= f32
impl SubAssign<f32> for Float2 {
    fn sub_assign(&mut self, v: f32) {
        *self = *self - v;
    }
}

// -Float2
impl Neg for Float2 {
    type Output = Float2;
 
    fn neg(self) -> Float2 {
        return Float2::new(-self.x, -self.y);
    }
}

// Vec3 * f32
impl Mul<f32> for Float2 {
    type Output = Float2;
 
    fn mul(self, v: f32) -> Float2 {
        return Float2::new(self.x * v, self.y * v);
    }
}

// f32 * Vec3
impl Mul<Float2> for f32 {
    type Output = Float2;
 
    fn mul(self, v: Float2) -> Float2 {
        return Float2::new(self * v.x, self * v.y);
    }
}
 
// Float2 *= f32
impl MulAssign<f32> for Float2 {
    fn mul_assign(&mut self, t: f32) {
        *self = *self * t;
    }
}

// Vec3 / f32
impl Div<f32> for Float2 {
    type Output = Float2;
 
    fn div(self, v: f32) -> Float2 {
        return Float2::new(self.x / v, self.y / v);
    }
}