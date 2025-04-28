pub mod float2;
pub mod float3;
pub mod float4;

pub mod bivector3;
pub mod bivector4;

pub mod rotor3;
pub mod rotor4;


#[allow(dead_code)]
const DEGREES_TO_RADIANS: f32 =  0.01745329;

#[allow(dead_code)]
const RADIANS_TO_DEGREES: f32 = 57.29577951;

#[allow(dead_code)]
const PI: f32 = 3.14159265358979323846264338327950;

#[allow(dead_code)]
pub fn clamp(x: f32, min: f32, max: f32) -> f32
{
    if x < min
    {
        return min;
    }
    if x > max
    {
        return max;
    }
    return x;
}

#[allow(dead_code)]
pub fn approx_equal(x: f32, y: f32) -> bool 
{ 
    return f32::abs(x-y) < 1e-6; 
}
