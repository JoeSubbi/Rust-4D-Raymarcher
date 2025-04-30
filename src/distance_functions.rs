use crate::mathematics::vector::VectorGrade1;

pub fn sdf_sphere<T: VectorGrade1>(p: T, centre: T, radius: f32) -> f32
{
    return (p-centre).length() - radius;
}