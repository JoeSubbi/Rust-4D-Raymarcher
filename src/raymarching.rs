use crate::mathematics::float3::Float3;
use crate::mathematics::float4::Float4;
use crate::mathematics::multivectors::{Magnitude, Vector};

pub const MAX_DIST: f32 = 100.0;
const MAX_STEPS: i32 = 100;
const SURF_DIST: f32 = 0.001;

pub fn raymarch<V: Vector>(ro: &V, rd: &V, distance_function: fn(V) -> f32) -> f32
{
    let mut d_origin: f32 = 0.0; // Distance from Origin

    for _i in 0..MAX_STEPS
    {
        let p: V = *ro + (*rd * d_origin);
        let d_surface: f32  = distance_function(p);
        d_origin += d_surface;
        if d_surface < SURF_DIST || d_origin > MAX_DIST
        {
            break;
        }
    }

    return d_origin;
}

pub fn normal3(p: Float3, distance_function: fn(Float3) -> f32) -> Float3
{
    const E: f32 = 0.01;
    let n: Float3 = distance_function(p) - Float3::new(
        distance_function(p - Float3::new(E, 0.0, 0.0)),
        distance_function(p - Float3::new(0.0, E, 0.0)),
        distance_function(p - Float3::new(0.0, 0.0, E))
    );
    
    if n.length_squared() == 0.0
    {
        return n;
    }

    return n.normalized();
}

pub fn normal4(p: Float4, distance_function: fn(Float4) -> f32) -> Float4
{
    const E: f32 = 0.01;
    let n: Float4 = distance_function(p) - Float4::new(
        distance_function(p - Float4::new(E, 0.0, 0.0, 0.0)),
        distance_function(p - Float4::new(0.0, E, 0.0, 0.0)),
        distance_function(p - Float4::new(0.0, 0.0, E, 0.0)),
        distance_function(p - Float4::new(0.0, 0.0, 0.0, E))
    );
    
    if n.length_squared() == 0.0
    {
        return n;
    }

    return n.normalized();
}
