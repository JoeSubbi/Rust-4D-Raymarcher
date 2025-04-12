pub mod float2;
pub mod float3;
pub mod float4;

#[allow(dead_code)]
const DEGREES_TO_RADIANS: f32 =  0.01745329;

#[allow(dead_code)]
const RADIANS_TO_DEGREES: f32 = 57.29577951;

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