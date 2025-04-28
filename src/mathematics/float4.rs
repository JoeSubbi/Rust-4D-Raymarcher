use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::mathematics::approx_equal;
use crate::mathematics::bivector4::Bivector4;

#[derive(Copy, Clone, Debug, Default)]
pub struct Float4
{
    pub x : f32,
    pub y : f32,
    pub z : f32,
    pub w : f32,
}

impl Float4
{
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Float4
    {
        return Float4{ x: x, y: y, z: z, w: w};
    }

    pub fn length(&self) -> f32
    {
        let length : f32 = self.length_squared();
        return f32::sqrt(length);
    }

    pub fn length_squared(&self) -> f32
    {
        return self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
    }

    pub fn normalize(&mut self)
    {
        let length : f32 = self.length();
        if length > 0.0
        {
            *self = *self / length;
        }
    }

    pub fn normalized(&self) -> Float4
    {
        let length : f32 = self.length();
        if length > 0.0
        {
            return *self / length;
        }
        else {
            return *self;
        }
    }
    
    pub fn dot(u: Float4, v: Float4) -> f32
    {
        return u.x * v.x + u.y * v.y + u.z * v.z + u.w * v.w;
    }

    pub fn wedge(u: Float4, v: Float4) -> Bivector4
    {
        return Bivector4{
            xy: u.x * v.y - u.y * v.x,
            xz: u.x * v.z - u.z * v.x,
            yz: u.y * v.z - u.z * v.y,
            xw: u.x * v.w - u.w * v.x,
            yw: u.y * v.w - u.w * v.y,
            zw: u.z * v.w - u.w * v.z
        };
    }

}

// Output formatting
impl Display for Float4 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {} {}", self.x, self.y, self.z, self.w)
    }
}

// Float4 + Float4
impl Add for Float4 {
    type Output = Float4;
 
    fn add(self, v: Float4) -> Float4 {
        return Float4::new(self.x + v.x, self.y + v.y, self.z + v.z, self.w + v.w);
    }
}

// Float4 + f32
impl Add<f32> for Float4 {
    type Output = Float4;
 
    fn add(self, v: f32) -> Float4 {
        return Float4::new(self.x + v, self.y + v, self.z + v, self.w + v);
    }
}

// Float4 += Float4
impl AddAssign for Float4 {
    fn add_assign(&mut self, v: Float4) {
        *self = *self + v;
    }
}

// Float4 += f32
impl AddAssign<f32> for Float4 {
    fn add_assign(&mut self, v: f32) {
        *self = *self + v;
    }
}
 
// Float4 - Float4
impl Sub for Float4 {
    type Output = Float4;
 
    fn sub(self, v: Float4) -> Float4 {
        return Float4::new(self.x - v.x, self.y - v.y, self.z - v.z, self.w - v.w);
    }
}

// Float4 - f32
impl Sub<f32> for Float4 {
    type Output = Float4;
 
    fn sub(self, v: f32) -> Float4 {
        return Float4::new(self.x - v, self.y - v, self.z - v, self.w - v);
    }
}

// Float4 -= Float4
impl SubAssign for Float4 {
    fn sub_assign(&mut self, v: Float4) {
        *self = *self - v;
    }
}

// Float4 -= f32
impl SubAssign<f32> for Float4 {
    fn sub_assign(&mut self, v: f32) {
        *self = *self - v;
    }
}

// -Float4
impl Neg for Float4 {
    type Output = Float4;
 
    fn neg(self) -> Float4 {
        return Float4::new(-self.x, -self.y, -self.z, -self.w);
    }
}

// Vec3 * f32
impl Mul<f32> for Float4 {
    type Output = Float4;
 
    fn mul(self, v: f32) -> Float4 {
        return Float4::new(self.x * v, self.y * v, self.z * v, self.w * v);
    }
}

// f32 * Vec3
impl Mul<Float4> for f32 {
    type Output = Float4;
 
    fn mul(self, v: Float4) -> Float4 {
        return Float4::new(self * v.x, self * v.y, self * v.z, self * v.w);
    }
}
 
// Float4 *= f32
impl MulAssign<f32> for Float4 {
    fn mul_assign(&mut self, t: f32) {
        *self = *self * t;
    }
}

// Vec3 / f32
impl Div<f32> for Float4 {
    type Output = Float4;
 
    fn div(self, v: f32) -> Float4 {
        return Float4::new(self.x / v, self.y / v, self.z / v, self.w / v);
    }
}

impl PartialEq for Float4
{
    fn eq(&self, other: &Self) -> bool 
    {
        return approx_equal(self.x, other.x) && 
               approx_equal(self.y, other.y) && 
               approx_equal(self.z, other.z) && 
               approx_equal(self.w, other.w);
    }
}