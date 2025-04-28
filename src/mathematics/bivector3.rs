use std::cmp::PartialEq;
use std::ops::Neg;

use crate::mathematics::approx_equal;

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