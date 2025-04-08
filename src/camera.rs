use crate::mathematics::float2::Float2;
use crate::mathematics::float3::Float3;

pub struct Camera
    {
        pub position: Float3,
        viewport_dimensions: Float2,
        lower_left_corner: Float3,
    }

    impl Camera
    {
        pub fn new(position: Float3, aspect_ratio: f32, viewport_height: f32, focal_length: f32) -> Camera
        {
            let viewport_width: f32 = aspect_ratio * viewport_height;
        
            let lower_left_corner: Float3 = Float3::new(
                position.x - viewport_width * 0.5,
                position.y - viewport_height * 0.5,
                position.z - focal_length
            ); 

            return Camera{
                position: position,
                viewport_dimensions: Float2{x: viewport_width, y: viewport_height},
                lower_left_corner: lower_left_corner,
            };
        }

        pub fn get_ray_direction(&self, uv: Float2) -> Float3 {

            return self.lower_left_corner + Float3::new(uv.x * self.viewport_dimensions.x, uv.y * self.viewport_dimensions.y, 0.0) - self.position;
        }
    }