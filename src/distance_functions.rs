
use crate::mathematics::float3::Float3;
use crate::mathematics::float4::Float4;
use crate::mathematics::multivectors::Magnitude;
use crate::mathematics::multivectors::Vector;

#[allow(dead_code)]
pub fn sdf_sphere<T: Vector>(p: T, centre: T, radius: f32) -> f32
{
    return (p-centre).length() - radius;
}

#[allow(dead_code)]
pub fn sdf_box3(p: Float3, centre: Float3, size: Float3) -> f32
{
    let p: Float3 = p-centre;
    let q: Float3 = Float3::new(f32::abs(p.x), f32::abs(p.y), f32::abs(p.z)) - size;
    
    return 
        Float3::new(f32::max(q.x, 0.0), f32::max(q.y, 0.0), f32::max(q.z, 0.0)).length() + 
        f32::min(
            f32::max(q.x, 
                f32::max(q.y, q.z)
            ),
            0.0
        )
}

#[allow(dead_code)]
pub fn sdf_box4(p: Float4, centre: Float4, size: Float4) -> f32
{
    let p: Float4 = p-centre;
    let q: Float4 = Float4::new(f32::abs(p.x), f32::abs(p.y), f32::abs(p.z), f32::abs(p.w)) - size;
    
    return 
        Float4::new(f32::max(q.x, 0.0), f32::max(q.y, 0.0), f32::max(q.z, 0.0), f32::max(q.w, 0.0)).length() + 
        f32::min(
            f32::max(q.x, 
                f32::max(q.y, 
                    f32::max(q.z, q.w)
                )
            ),
            0.0
        );
}