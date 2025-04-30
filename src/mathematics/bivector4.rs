use std::cmp::PartialEq;
use std::ops::Neg;

use super::approx_equal;
use super::multivectors::Bivector;

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