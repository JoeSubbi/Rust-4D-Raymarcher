use std::ops::{Mul, MulAssign};

#[derive(Copy, Clone, Default)]
pub struct rotor4
{
    a : f32,
    bv : bivector4,
    p : f32,
}