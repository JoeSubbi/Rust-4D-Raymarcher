use std::ops::{Mul, MulAssign};

#[derive(Copy, Clone, Default)]
pub struct Rotor4
{
    a : f32,
    bv : Bivector4,
    p : f32,
}