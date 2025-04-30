use crate::mathematics::multivectors::Vector;

pub fn sdf_sphere<T: Vector>(p: T, centre: T, radius: f32) -> f32
{
    return (p-centre).length() - radius;
}