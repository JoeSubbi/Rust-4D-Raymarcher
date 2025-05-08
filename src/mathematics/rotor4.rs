use std::cmp::PartialEq;
use std::ops::{Mul, MulAssign};

use super::approx_equal;
use super::bivector4::Bivector4;
use super::float4::Float4;
use super::multivectors::{Magnitude, Rotor, Vector};

#[derive(Copy, Clone, Debug, Default)]
pub struct Rotor4
{
    a : f32,
    bv : Bivector4,
    p : f32,
}

impl Rotor4
{
    pub const IDENTITY: Rotor4 = Rotor4{ a: 1.0, bv: Bivector4{yz: 0.0, xz: 0.0, xy: 0.0, xw: 0.0, yw: 0.0, zw: 0.0}, p: 0.0 };

    pub fn new(a: f32, bv: Bivector4, p: f32) -> Rotor4
    {
        return Rotor4{ a: a, bv: bv, p: p};
    }
}

impl Rotor<Float4, Bivector4> for Rotor4
{
    /// Creates a new rotor in the specified bivector given an angle in radians
    fn bivector_angle(bv: &Bivector4, angle: f32) -> Rotor4
    {
        let bv = bv.normalized();
        
        let sina: f32 = f32::sin(angle / 2.0);

        return Rotor4{
            a: f32::cos(angle / 2.0),
            bv: Bivector4{
                yz: -sina * bv.yz,
                xz: -sina * bv.xz,
                xy: -sina * bv.xy,
                xw: -sina * bv.xw,
                yw: -sina * bv.yw,
                zw: -sina * bv.zw,
            },
            p: 0.0
        }.normalized();
    }

    fn geometric_product(u: Float4, v: Float4) -> Rotor4
    {
        return Rotor4 {
            a: Float4::dot(u, v),
            bv: Float4::wedge(u, v),
            p: 0.0
        };
    }

    fn reverse(r: &Rotor4) -> Rotor4
    {
        return Rotor4::new(r.a, -r.bv, r.p);
    }

    fn reverse_me(&mut self)
    {
        self.bv = -self.bv;
    }
    
    fn rotate_rotor(a: &Rotor4, b: &Rotor4) -> Rotor4
    {
        let e: f32     = -a.bv.xw * b.bv.xw   - a.bv.xy * b.bv.xy   - a.bv.xz * b.bv.xz   - a.bv.yw * b.bv.yw   - a.bv.yz * b.bv.yz   - a.bv.zw * b.bv.zw   + a.p * b.p       + a.a * b.a;
        let exy: f32   = -a.bv.xw * b.bv.yw   + a.bv.xy * b.a       - a.bv.xz * b.bv.yz   + a.bv.yw * b.bv.xw   + a.bv.yz * b.bv.xz   - a.bv.zw * b.p       - a.p * b.bv.zw   + a.a * b.bv.xy;
        let exz: f32   = -a.bv.xw * b.bv.zw   + a.bv.xy * b.bv.yz   + a.bv.xz * b.a       + a.bv.yw * b.p       - a.bv.yz * b.bv.xy   + a.bv.zw * b.bv.xw   + a.p * b.bv.yw   + a.a * b.bv.xz;
        let exw: f32   =  a.bv.xw * b.a       + a.bv.xy * b.bv.yw   + a.bv.xz * b.bv.zw   - a.bv.yw * b.bv.xy   - a.bv.yz * b.p       - a.bv.zw * b.bv.xz   - a.p * b.bv.yz   + a.a * b.bv.xw;
        let eyz: f32   = -a.bv.xw * b.p       - a.bv.xy * b.bv.xz   + a.bv.xz * b.bv.xy   - a.bv.yw * b.bv.zw   + a.bv.yz * b.a       + a.bv.zw * b.bv.yw   - a.p * b.bv.xw   + a.a * b.bv.yz;
        let eyw: f32   =  a.bv.xw * b.bv.xy   - a.bv.xy * b.bv.xw   + a.bv.xz * b.p       + a.bv.yw * b.a       + a.bv.yz * b.bv.zw   - a.bv.zw * b.bv.yz   + a.p * b.bv.xz   + a.a * b.bv.yw;
        let ezw: f32   =  a.bv.xw * b.bv.xz   - a.bv.xy * b.p       - a.bv.xz * b.bv.xw   + a.bv.yw * b.bv.yz   - a.bv.yz * b.bv.yw   + a.bv.zw * b.a       - a.p * b.bv.xy   + a.a * b.bv.zw;
        let exyzw: f32 =  a.bv.xw * b.bv.yz   + a.bv.xy * b.bv.zw   - a.bv.xz * b.bv.yw   - a.bv.yw * b.bv.xz   + a.bv.yz * b.bv.xw   + a.bv.zw * b.bv.xy   + a.p * b.a       + a.a * b.p;

        return Rotor4{ 
            a: e, 
            bv: Bivector4 { 
                yz: eyz, 
                xz: exz, 
                xy: exy, 
                xw: exw, 
                yw: eyw, 
                zw: ezw 
            },
            p: exyzw
        };
    }

    fn rotate_vector(&self, v: Float4) -> Float4
    {
        let a: f32 = self.a;
        let a2: f32 = a * a;
        let bxy2: f32 = self.bv.xy * self.bv.xy;
        let bxz2: f32 = self.bv.xz * self.bv.xz;
        let bxw2: f32 = self.bv.xw * self.bv.xw;
        let byz2: f32 = self.bv.yz * self.bv.yz;
        let byw2: f32 = self.bv.yw * self.bv.yw;
        let bzw2: f32 = self.bv.zw * self.bv.zw;
        let p2: f32 = self.p * self.p;

        let r: Float4 = Float4{
            x: (
                  2.0 * v.w * self.bv.xw * a
                + 2.0 * v.w * self.bv.xy * self.bv.yw
                + 2.0 * v.w * self.bv.xz * self.bv.zw
                + 2.0 * v.w * self.bv.yz * self.p
                - v.x * bxw2
                - v.x * bxy2
                - v.x * bxz2
                + v.x * byw2
                + v.x * byz2
                + v.x * bzw2
                - v.x * p2
                + v.x * a2
                - 2.0 * v.y * self.bv.xw * self.bv.yw
                + 2.0 * v.y * self.bv.xy * a
                - 2.0 * v.y * self.bv.xz * self.bv.yz
                + 2.0 * v.y * self.bv.zw * self.p
                - 2.0 * v.z * self.bv.xw * self.bv.zw
                + 2.0 * v.z * self.bv.xy * self.bv.yz
                + 2.0 * v.z * self.bv.xz * a
                - 2.0 * v.z * self.bv.yw * self.p
            ),

            y: (
                - 2.0 * v.w * self.bv.xw * self.bv.xy
                - 2.0 * v.w * self.bv.xz * self.p
                + 2.0 * v.w * self.bv.yw * a
                + 2.0 * v.w * self.bv.yz * self.bv.zw
                - 2.0 * v.x * self.bv.xw * self.bv.yw
                - 2.0 * v.x * self.bv.xy * a
                - 2.0 * v.x * self.bv.xz * self.bv.yz
                - 2.0 * v.x * self.bv.zw * self.p
                + v.y * bxw2
                - v.y * bxy2
                + v.y * bxz2
                - v.y * byw2
                - v.y * byz2
                + v.y * bzw2
                - v.y * p2
                + v.y * a2
                + 2.0 * v.z * self.bv.xw * self.p
                - 2.0 * v.z * self.bv.xy * self.bv.xz
                - 2.0 * v.z * self.bv.yw * self.bv.zw
                + 2.0 * v.z * self.bv.yz * a
            ),

            z: (
                - 2.0 * v.w * self.bv.xw * self.bv.xz
                + 2.0 * v.w * self.bv.xy * self.p
                - 2.0 * v.w * self.bv.yw * self.bv.yz
                + 2.0 * v.w * self.bv.zw * a
                - 2.0 * v.x * self.bv.xw * self.bv.zw
                + 2.0 * v.x * self.bv.xy * self.bv.yz
                - 2.0 * v.x * self.bv.xz * a
                + 2.0 * v.x * self.bv.yw * self.p
                - 2.0 * v.y * self.bv.xw * self.p
                - 2.0 * v.y * self.bv.xy * self.bv.xz
                - 2.0 * v.y * self.bv.yw * self.bv.zw
                - 2.0 * v.y * self.bv.yz * a
                + v.z * bxw2
                + v.z * bxy2
                - v.z * bxz2
                + v.z * byw2
                - v.z * byz2
                - v.z * bzw2
                - v.z * p2
                + v.z * a2
    
            ),

            w: (
                - v.w * bxw2
                + v.w * bxy2
                + v.w * bxz2
                - v.w * byw2
                + v.w * byz2
                - v.w * bzw2
                - v.w * p2
                + v.w * a2
                - 2.0 * v.x * self.bv.xw * a
                + 2.0 * v.x * self.bv.xy * self.bv.yw
                + 2.0 * v.x * self.bv.xz * self.bv.zw
                - 2.0 * v.x * self.bv.yz * self.p
                - 2.0 * v.y * self.bv.xw * self.bv.xy
                + 2.0 * v.y * self.bv.xz * self.p
                - 2.0 * v.y * self.bv.yw * a
                + 2.0 * v.y * self.bv.yz * self.bv.zw
                - 2.0 * v.z * self.bv.xw * self.bv.xz
                - 2.0 * v.z * self.bv.xy * self.p
                - 2.0 * v.z * self.bv.yw * self.bv.yz
                - 2.0 * v.z * self.bv.zw * a
            )
        };

        return r;
    }

    fn slerp(from: &Rotor4, to: &Rotor4, ratio: f32) -> Rotor4
    {
        // The following SLerp is from:
        // https://referencesource.microsoft.com/#System.Numerics/System/Numerics/Quaternion.cs

        const EPSILON: f32 = 1e-6;

        let t: f32 = ratio;

        let difference: Rotor4 = Rotor4::from_to(from, to);
        let mut cos_omega: f32 = f32::sqrt(difference.a * difference.a + difference.p * difference.p);

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

        let mut to_return: Rotor4 = Rotor4{
            a: s1 * from.a + s2 * to.a,
            bv: Bivector4{
                yz: s1 * from.bv.yz + s2 * to.bv.yz,
                xy: s1 * from.bv.xy + s2 * to.bv.xy,
                xz: s1 * from.bv.xz + s2 * to.bv.xz,
                xw: s1 * from.bv.xw + s2 * to.bv.xw,
                yw: s1 * from.bv.yw + s2 * to.bv.yw,
                zw: s1 * from.bv.zw + s2 * to.bv.zw
            },
            p: s1 * from.p + s2 * to.p
        };

        to_return.normalize();

        return to_return;
    }
    
    /// Gives the angle of this Rotor in Radians
    fn angle(&self) -> f32
    {
        return f32::acos(f32::sqrt(self.a * self.a + self.p * self.p) ) * 2.0;
    }
}

impl Magnitude for Rotor4
{
    fn length_squared(&self) -> f32
    {
        return self.a * self.a + self.bv.yz * self.bv.yz + self.bv.xz * self.bv.xz + self.bv.xy * self.bv.xy +
               self.bv.xw * self.bv.xw + self.bv.yw * self.bv.yw + self.bv.zw * self.bv.zw + self.p * self.p;
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
            self.bv.xw /= length;
            self.bv.yw /= length;
            self.bv.zw /= length;
            self.p /= length;
        }
    }

    fn normalized(&self) -> Rotor4
    {
        let length : f32 = self.length();
        if length > 0.0
        {
            return Rotor4{
                a: self.a / length,
                bv: Bivector4 { 
                    yz: self.bv.yz / length, 
                    xz: self.bv.xz / length, 
                    xy: self.bv.xy / length, 
                    xw: self.bv.xw / length, 
                    yw: self.bv.yw / length, 
                    zw: self.bv.zw / length
                },
                p: self.p / length
            };
        }
        else 
        {
            return *self;
        }
    }
}

// Rotor4 * Float4
impl Mul<Float4> for Rotor4
{
    type Output = Float4;
    
    fn mul(self, v: Float4) -> Float4
    {
        return self.rotate_vector(v);
    }
}

// Float4 * Rotor4 
impl Mul<Rotor4> for Float4
{
    type Output = Float4;
    
    fn mul(self, r: Rotor4) -> Float4
    {
        return r.rotate_vector(self);
    }
}

// Rotor4 * Rotor4
impl Mul for Rotor4
{
    type Output = Rotor4;
    
    fn mul(self, other: Rotor4) -> Rotor4
    {
        return Rotor4::rotate_rotor(&self, &other);
    }
}

// Rotor4 *= Rotor4
impl MulAssign for Rotor4
{
    fn mul_assign(&mut self, rhs: Self) 
    {
        *self = *self * rhs;
    }
}

impl PartialEq for Rotor4
{
    fn eq(&self, other: &Self) -> bool 
    {
        return approx_equal(self.a, other.a) && 
               self.bv == other.bv &&
               approx_equal(self.p, other.p);
    }
}


#[cfg(test)]
#[path = "rotor4_tests.rs"]
mod tests;