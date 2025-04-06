use std::ops::{Mul, MulAssign};

#[derive(Copy, Clone, Default)]
pub struct Rotor3
{
    a : f32,
    bv : Bivector3,
}