use std::cmp::PartialEq;
use std::ops::Neg;

use super::approx_equal;
use super::multivectors::{Bivector, Magnitude};

#[derive(Copy, Clone, Debug, Default)]
pub struct Bivector3
{
    pub yz : f32,
    pub xz : f32,
    pub xy : f32,
}

impl Bivector3
{
    pub fn new(yz: f32, xz: f32, xy: f32) -> Bivector3
    {
        return Bivector3{ yz: yz, xz: xz, xy: xy };
    }
}

impl Bivector for Bivector3 {}

impl Magnitude for Bivector3
{
    fn length_squared(&self) -> f32
    {
        return self.yz * self.yz + self.xz * self.xz + self.xy * self.xy;
    }

    fn normalize(&mut self)
    {
        let length : f32 = self.length();
        if length > 0.0
        {
            self.yz /= length;
            self.xz /= length;
            self.xy /= length;
        }
    }

    fn normalized(&self) -> Bivector3
    {
        let length : f32 = self.length();
        if length > 0.0
        {
            return Bivector3 { 
                yz: self.yz / length, 
                xz: self.xz / length, 
                xy: self.xy / length
            };
        }
        else 
        {
            return *self;
        }
    }
}

impl Neg for Bivector3 {
    type Output = Bivector3;
 
    fn neg(self) -> Bivector3 {
        return Bivector3{yz: -self.yz, xz: -self.xz, xy: -self.xy};
    }
}

impl PartialEq for Bivector3
{
    fn eq(&self, other: &Self) -> bool 
    {
        return approx_equal(self.yz,other.yz) &&
               approx_equal(self.xz, other.xz) &&
               approx_equal(self.xy, other.xy);
    }
}