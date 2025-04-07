use super::SDF;
use crate::mathematics::float3::Float3;

pub struct Sphere
{
    centre: Float3,
    radius: f32,
}

impl Sphere
{
    pub fn new(centre: Float3, radius: f32) -> Sphere
    {
        return Sphere{ centre: centre, radius: radius };
    }
}

impl SDF for Sphere
{
    fn sdf(&self, p: Float3) -> f32 {
        return (p-self.centre).length() - self.radius;
    }
}