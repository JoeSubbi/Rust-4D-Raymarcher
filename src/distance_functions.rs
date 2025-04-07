use crate::mathematics::float3::Float3;

pub fn sdf_sphere(p: Float3, centre: Float3, radius: f32) -> f32
{
    return (p-centre).length() - radius;
}