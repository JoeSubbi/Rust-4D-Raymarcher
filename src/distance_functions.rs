use crate::mathematics::float3::Float3;
    
pub fn sphere(p: Float3, r: f32) -> f32
{
    return p.length() - r;
}