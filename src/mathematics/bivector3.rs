use std::ops::Neg;

#[derive(Copy, Clone, Default)]
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