use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

use super::approx_equal;
use super::bivector3::Bivector3;
use super::float2::Float2;
use super::float4::Float4;
use super::multivectors::{Magnitude, Vector};

#[derive(Copy, Clone, Debug, Default)]
pub struct Float3
{
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

impl Float3
{
    pub fn new(x: f32, y: f32, z: f32) -> Float3
    {
        return Float3{ x: x, y: y, z: z };
    }

    pub fn cross(u: Float3, v: Float3) -> Float3
    {
        return Float3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x
        );
    }

    pub fn wedge(u: Float3, v: Float3) -> Bivector3
    {
        return Bivector3{
            xy: u.x * v.y - u.y * v.x,
            xz: u.x * v.z - u.z * v.x,
            yz: u.y * v.z - u.z * v.y,
        };
    }
}

impl Vector for Float3 
{    
    fn dot(u: Float3, v: Float3) -> f32
    {
        return u.x * v.x + u.y * v.y + u.z * v.z;
    }
}

impl Magnitude for Float3
{
    fn length_squared(&self) -> f32
    {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    fn normalize(&mut self)
    {
        let length : f32 = self.length();
        if length > 0.0
        {
            *self = *self / length;
        }
    }

    fn normalized(&self) -> Float3
    {
        let length : f32 = self.length();
        if length > 0.0
        {
            return *self / length;
        }
        else 
        {
            return *self;
        }
    }
}

// Output formatting
impl Display for Float3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

// Float3 + Float3
impl Add for Float3 {
    type Output = Float3;
 
    fn add(self, v: Float3) -> Float3 {
        return Float3::new(self.x + v.x, self.y + v.y, self.z + v.z);
    }
}

// Float3 + f32
impl Add<f32> for Float3 {
    type Output = Float3;
 
    fn add(self, v: f32) -> Float3 {
        return Float3::new(self.x + v, self.y + v, self.z + v);
    }
}

// f32 + Float3
impl Add<Float3> for f32 {
    type Output = Float3;
 
    fn add(self, v: Float3) -> Float3 {
        return Float3::new(self + v.x, self + v.y, self + v.z);
    }
}

// Float3 += Float3
impl AddAssign for Float3 {
    fn add_assign(&mut self, v: Float3) {
        *self = *self + v;
    }
}

// Float3 += f32
impl AddAssign<f32> for Float3 {
    fn add_assign(&mut self, v: f32) {
        *self = *self + v;
    }
}
 
// Float3 - Float3
impl Sub for Float3 {
    type Output = Float3;
 
    fn sub(self, v: Float3) -> Float3 {
        return Float3::new(self.x - v.x, self.y - v.y, self.z - v.z);
    }
}

// Float3 - f32
impl Sub<f32> for Float3 {
    type Output = Float3;
 
    fn sub(self, v: f32) -> Float3 {
        return Float3::new(self.x - v, self.y - v, self.z - v);
    }
}

// f32 - Float3
impl Sub<Float3> for f32 {
    type Output = Float3;
 
    fn sub(self, v: Float3) -> Float3 {
        return Float3::new(self - v.x, self - v.y, self - v.z);
    }
}

// Float3 -= Float3
impl SubAssign for Float3 {
    fn sub_assign(&mut self, v: Float3) {
        *self = *self - v;
    }
}

// Float3 -= f32
impl SubAssign<f32> for Float3 {
    fn sub_assign(&mut self, v: f32) {
        *self = *self - v;
    }
}

// -Float3
impl Neg for Float3 {
    type Output = Float3;
 
    fn neg(self) -> Float3 {
        return Float3::new(-self.x, -self.y, -self.z);
    }
}

// Float3 * f32
impl Mul<f32> for Float3 {
    type Output = Float3;
 
    fn mul(self, v: f32) -> Float3 {
        return Float3::new(self.x * v, self.y * v, self.z * v);
    }
}

// f32 * Vec3
impl Mul<Float3> for f32 {
    type Output = Float3;
 
    fn mul(self, v: Float3) -> Float3 {
        return Float3::new(self * v.x, self * v.y, self * v.z);
    }
}
 
// Float3 *= f32
impl MulAssign<f32> for Float3 {
    fn mul_assign(&mut self, t: f32) {
        *self = *self * t;
    }
}

// Vec3 / f32
impl Div<f32> for Float3 {
    type Output = Float3;
 
    fn div(self, v: f32) -> Float3 {
        return Float3::new(self.x / v, self.y / v, self.z / v);
    }
}

impl PartialEq for Float3
{
    fn eq(&self, other: &Self) -> bool 
    {
        return approx_equal(self.x, other.x) && 
               approx_equal(self.y, other.y) && 
               approx_equal(self.z, other.z);
    }
}


impl From<Float2> for Float3
{
    fn from(item: Float2) -> Self
    {
        return Float3::new(item.x, item.y, 0.0);
    }
}

impl From<Float4> for Float3
{
    fn from(item: Float4) -> Self
    {
        return Float3::new(item.x, item.y, item.z);
    }
}