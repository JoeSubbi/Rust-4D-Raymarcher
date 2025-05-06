use std::cmp::PartialEq;
use std::ops::Neg;

use super::approx_equal;
use super::multivectors::{Bivector, Magnitude};

#[derive(Copy, Clone, Debug, Default)]
pub struct Bivector4
{
    pub yz : f32,
    pub xz : f32,
    pub xy : f32,
    pub xw : f32,
    pub yw : f32,
    pub zw : f32,
}

impl Bivector4
{
    pub fn new(yz: f32, xz: f32, xy: f32, xw: f32, yw: f32, zw: f32) -> Bivector4
    {
        return Bivector4{ yz: yz, xz: xz, xy: xy, xw: xw, yw: yw, zw: zw };
    }
}

impl Bivector for Bivector4 {}

impl Magnitude for Bivector4
{
    fn length_squared(&self) -> f32
    {
        return self.yz * self.yz + self.xz * self.xz + self.xy * self.xy +
               self.xw * self.xw + self.yw * self.yw + self.zw * self.zw;
    }

    fn normalize(&mut self)
    {
        let length : f32 = self.length();
        if length > 0.0
        {
            self.yz /= length;
            self.xz /= length;
            self.xy /= length;
            self.xw /= length;
            self.yw /= length;
            self.zw /= length;
        }
    }

    fn normalized(&self) -> Bivector4
    {
        let length : f32 = self.length();
        if length > 0.0
        {
            return Bivector4 { 
                yz: self.yz / length, 
                xz: self.xz / length, 
                xy: self.xy / length, 
                xw: self.xw / length, 
                yw: self.yw / length, 
                zw: self.zw / length
            };
        }
        else 
        {
            return *self;
        }
    }
}

impl Neg for Bivector4 {
    type Output = Bivector4;
 
    fn neg(self) -> Bivector4 {
        return Bivector4{yz: -self.yz, xz: -self.xz, xy: -self.xy, xw: -self.xw, yw: -self.yw, zw: -self.zw};
    }
}

impl PartialEq for Bivector4
{
    fn eq(&self, other: &Self) -> bool 
    {
        return approx_equal(self.yz,other.yz) &&
               approx_equal(self.xz, other.xz) &&
               approx_equal(self.xy, other.xy) &&
               approx_equal(self.xw, other.xw) &&
               approx_equal(self.yw, other.yw) &&
               approx_equal(self.zw, other.zw);
    }
}