use std::ops::{Mul, MulAssign};

#[derive(Copy, Clone, Default)]
pub struct rotor3
{
    a : f32,
    bv : bivector3,
}