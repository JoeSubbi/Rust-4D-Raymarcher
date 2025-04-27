use std::ops::Neg;

#[derive(Copy, Clone, Default)]
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

impl Neg for Bivector4 {
    type Output = Bivector4;
 
    fn neg(self) -> Bivector4 {
        return Bivector4{yz: -self.yz, xz: -self.xz, xy: -self.xy, xw: -self.xw, yw: -self.yw, zw: -self.zw};
    }
}