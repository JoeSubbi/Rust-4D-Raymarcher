use std::cmp::PartialEq;
use std::ops::{Mul, MulAssign};

use super::approx_equal;
use super::bivector3::Bivector3;
use super::float3::Float3;
use super::multivectors::{Magnitude, Rotor, Vector};

#[derive(Copy, Clone, Debug, Default)]
pub struct Rotor3
{
    a : f32,
    bv : Bivector3,
}

impl Rotor3
{
    pub const IDENTITY: Rotor3 = Rotor3{ a: 1.0, bv: Bivector3{yz: 0.0, xz: 0.0, xy: 0.0} };

    pub fn new(a: f32, bv: Bivector3) -> Rotor3
    {
        return Rotor3{ a: a, bv: bv };
    }
}

impl Rotor<Float3, Bivector3> for Rotor3
{
    /// Creates a new rotor in the specified bivector given an angle in radians
    fn bivector_angle(bv: &Bivector3, angle: f32) -> Rotor3
    {
        let bv = bv.normalized();
        
        let sina: f32 = f32::sin(angle / 2.0);

        return Rotor3{
            a: f32::cos(angle / 2.0),
            bv: Bivector3{
                yz: -sina * bv.yz,
                xz: -sina * bv.xz,
                xy: -sina * bv.xy
            }
        }.normalized();
    }

    fn geometric_product(u: Float3, v: Float3) -> Rotor3
    {
        return Rotor3 {
            a: Float3::dot(u, v),
            bv: Float3::wedge(u, v),
        };
    }

    fn reverse(r: &Rotor3) -> Rotor3
    {
        return Rotor3::new(r.a, -r.bv);
    }

    fn reverse_me(&mut self)
    {
        self.bv = -self.bv;
    }
    
    fn rotate_rotor(a: &Rotor3, b: &Rotor3) -> Rotor3
    {
        let e: f32   = a.a * b.a - a.bv.xy * b.bv.xy - a.bv.xz * b.bv.xz - a.bv.yz * b.bv.yz;
        let exy: f32 = a.bv.xy * b.a + a.a * b.bv.xy + a.bv.yz * b.bv.xz - a.bv.xz * b.bv.yz;
        let exz: f32 = a.bv.xz * b.a + a.a * b.bv.xz - a.bv.yz * b.bv.xy + a.bv.xy * b.bv.yz;
        let eyz: f32 = a.bv.yz * b.a + a.a * b.bv.yz + a.bv.xz * b.bv.xy - a.bv.xy * b.bv.xz;

        return Rotor3{ 
            a: e, 
            bv: Bivector3 { 
                yz: eyz, 
                xz: exz, 
                xy: exy
            }
        };
    }

    fn rotate_vector(&self, v: Float3) -> Float3
    {
        let s: f32 = self.a;
        let s2: f32 = s * s;
        let bxy2: f32 = self.bv.xy * self.bv.xy;
        let bxz2: f32 = self.bv.xz * self.bv.xz;
        let byz2: f32 = self.bv.yz * self.bv.yz;

        let r: Float3 = Float3 { 
            x: (
                - v.x * bxy2
                - v.x * bxz2
                + v.x * byz2
                + v.x * s2
                + 2.0 * v.y * self.bv.xy * s
                - 2.0 * v.y * self.bv.xz * self.bv.yz
                + 2.0 * v.z * self.bv.xy * self.bv.yz
                + 2.0 * v.z * self.bv.xz * s
            ), 
            y: (
                - 2.0 * v.x * self.bv.xy * s
                - 2.0 * v.x * self.bv.xz * self.bv.yz
                - v.y * bxy2
                + v.y * bxz2
                - v.y * byz2
                + v.y * s2
                - 2.0 * v.z * self.bv.xy * self.bv.xz
                + 2.0 * v.z * self.bv.yz * s
            ), 
            z: (
                  2.0 * v.x * self.bv.xy * self.bv.yz
                - 2.0 * v.x * self.bv.xz * s
                - 2.0 * v.y * self.bv.xy * self.bv.xz
                - 2.0 * v.y * self.bv.yz * s
                + v.z * bxy2
                - v.z * bxz2
                - v.z * byz2
                + v.z * s2
            )
        };

        return r;
    }

    fn slerp(from: &Rotor3, to: &Rotor3, ratio: f32) -> Rotor3
    {
        // The following SLerp is from:
        // https://referencesource.microsoft.com/#System.Numerics/System/Numerics/Quaternion.cs

        const EPSILON: f32 = 1e-6;

        let t: f32 = ratio;

        let difference: Rotor3 = Rotor3::from_to(from, to);
        let mut cos_omega: f32 = difference.a;

        let mut flip: bool = false;
 
        if cos_omega < 0.0
        {
            flip = true;
            cos_omega = -cos_omega;
        }
 
        let s1: f32; 
        let s2 : f32;
 
        if cos_omega > (1.0 - EPSILON)
        {
            // Too close, do straight linear interpolation.
            s1 = 1.0 - t;
            s2 = if flip { -t } else { t };
        }
        else
        {
            let omega: f32 = f32::acos(cos_omega);
            let inv_sin_omega: f32 = 1.0 / f32::sin(omega);
 
            s1 = f32::sin((1.0 - t) * omega) * inv_sin_omega;
            s2 = if flip { -f32::sin(t * omega) * inv_sin_omega } else { f32::sin(t * omega) * inv_sin_omega };
        }

        let mut to_return: Rotor3 = Rotor3{
            a: s1 * from.a + s2 * to.a,
            bv: Bivector3{
                yz: s1 * from.bv.yz + s2 * to.bv.yz,
                xy: s1 * from.bv.xy + s2 * to.bv.xy,
                xz: s1 * from.bv.xz + s2 * to.bv.xz
            }
        };

        to_return.normalize();

        return to_return;
    }

    /// Gives the angle of this Rotor in Radians
    fn angle(&self) -> f32
    {
        return f32::acos(self.a) * 2.0;
    }

    fn from_to(a: &Rotor3, b: &Rotor3) -> Rotor3
    {
        let difference: Rotor3 = *b * Rotor3::reverse(a);
        difference.normalized();
        return difference;
    }
    
    fn approx_equal(a: &Rotor3, b: &Rotor3) -> bool
    {
        let angle: f32 = Rotor3::from_to(a, b).angle();
        return angle < 0.001;
    }
}

impl Magnitude for Rotor3
{
    fn length_squared(&self) -> f32
    {
        return self.a * self.a + self.bv.yz * self.bv.yz + self.bv.xz * self.bv.xz + self.bv.xy * self.bv.xy;
    }

    fn normalize(&mut self)
    {
        let length : f32 = self.length();
        if length > 0.0
        {
            self.a /= length;
            self.bv.yz /= length;
            self.bv.xz /= length;
            self.bv.xy /= length;
        }
    }

    fn normalized(&self) -> Rotor3
    {
        let length : f32 = self.length();
        if length > 0.0
        {
            return Rotor3{
                a: self.a / length,
                bv: Bivector3 { 
                    yz: self.bv.yz / length, 
                    xz: self.bv.xz / length, 
                    xy: self.bv.xy / length
                }
            };
        }
        else 
        {
            return *self;
        }
    }
}

// Rotor3 * Float3
impl Mul<Float3> for Rotor3
{
    type Output = Float3;
    
    fn mul(self, v: Float3) -> Float3
    {
        return self.rotate_vector(v);
    }
}

// Float3 * Rotor3
impl Mul<Rotor3> for Float3
{
    type Output = Float3;
    
    fn mul(self, r: Rotor3) -> Float3
    {
        return r.rotate_vector(self);
    }
}

// Rotor3 * Rotor3
impl Mul for Rotor3
{
    type Output = Rotor3;
    
    fn mul(self, other: Rotor3) -> Rotor3
    {
        return Rotor3::rotate_rotor(&self, &other);
    }
}

// Rotor3 *= Rotor3
impl MulAssign for Rotor3
{
    fn mul_assign(&mut self, rhs: Self) 
    {
        *self = *self * rhs;
    }
}

impl PartialEq for Rotor3
{
    fn eq(&self, other: &Self) -> bool 
    {
        return approx_equal(self.a, other.a) && 
               self.bv == other.bv;
    }
}


#[cfg(test)]
#[path = "rotor3_tests.rs"]
mod tests;