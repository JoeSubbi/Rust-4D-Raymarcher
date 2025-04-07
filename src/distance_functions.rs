pub mod sphere;

use crate::mathematics::float3::Float3;
pub trait SDF
{
    fn sdf(&self, p: Float3) -> f32;
}