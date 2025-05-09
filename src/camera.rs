use crate::mathematics;
use crate::mathematics::bivector3::Bivector3;
use crate::mathematics::bivector4::Bivector4;
use crate::mathematics::float2::Float2;
use crate::mathematics::float3::Float3;
use crate::mathematics::float4::Float4;
use crate::mathematics::rotor3::Rotor3;
use crate::mathematics::rotor4::Rotor4;
use crate::mathematics::multivectors::{Bivector, Rotor, Vector};


pub trait Camera<V: Vector, B: Bivector, R: Rotor<V, B>>
{
    fn calculate_camera_lower_left_corner(position: &V, rotation: &R, viewport_dimensions: &Float3) -> V;
    
    fn recalculate(&mut self);

    fn new(pivot: V, offset_position: V, rotation: R, aspect_ratio: f32, vfov: f32, focal_length: f32) -> Self;

    fn get_camera_position(&self) -> V;

    fn get_ray_direction(&self, uv: Float2) -> V;

    #[allow(dead_code)]
    fn set_camera_rotation(&mut self, rotation: R);

    #[allow(dead_code)]
    fn rotate_camera(&mut self, rotation: R);
}

pub struct Camera3
{
    pivot: Float3,
    offset_position: Float3,
    position: Float3,
    rotation: Rotor3,
    viewport_dimensions: Float3,
    horizontal: Float3,
    vertical: Float3,
    lower_left_corner: Float3,
}

impl Camera<Float3, Bivector3, Rotor3> for Camera3 
{
    fn calculate_camera_lower_left_corner(position: &Float3, rotation: &Rotor3, viewport_dimensions: &Float3) -> Float3
    {
        let centre_offset_dimensions = Float3::new(viewport_dimensions.x * 0.5,  viewport_dimensions.y * 0.5, viewport_dimensions.z);
        let lower_left_corner: Float3 = *position - (*rotation * centre_offset_dimensions);
        return lower_left_corner;
    }

    fn recalculate(&mut self)
    {
        self.position = self.pivot + (self.rotation * self.offset_position);
        self.horizontal = self.rotation * Float3::new(self.viewport_dimensions.x, 0.0, 0.0);
        self.vertical = self.rotation * Float3::new(0.0, self.viewport_dimensions.y, 0.0);
        self.lower_left_corner = Self::calculate_camera_lower_left_corner(&self.position, &self.rotation, &self.viewport_dimensions);
    }

    fn new(pivot: Float3, offset_position: Float3, rotation: Rotor3, aspect_ratio: f32, vfov: f32, focal_length: f32) -> Self
    {
        let position: Float3 = pivot + (rotation * offset_position);

        let theta: f32 = mathematics::DEGREES_TO_RADIANS * vfov;
        let h = f32::tan(theta * 0.5);
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let horizontal: Float3 = rotation * Float3::new(viewport_width, 0.0, 0.0);
        let vertical: Float3 = rotation * Float3::new(0.0, viewport_height, 0.0);
        let viewport_dimensions: Float3 = Float3{x: viewport_width, y: viewport_height, z: focal_length};

        let lower_left_corner: Float3 = Self::calculate_camera_lower_left_corner(&position, &rotation, &viewport_dimensions);

        return Self{
            pivot: pivot,
            offset_position: offset_position,
            position: position,
            rotation: rotation,
            viewport_dimensions: viewport_dimensions,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
        };
    }

    fn get_camera_position(&self) -> Float3
    {
        return self.position;
    }

    fn get_ray_direction(&self, uv: Float2) -> Float3 
    {
        return self.lower_left_corner + uv.x * self.horizontal + uv.y * self.vertical - self.position;
    }

    fn set_camera_rotation(&mut self, rotation: Rotor3)
    {
        self.rotation = rotation;
        self.recalculate(); 
    }

    fn rotate_camera(&mut self, rotation: Rotor3)
    {
        self.rotation *= rotation;
        self.recalculate(); 
    }
}

pub struct Camera4
{
    pivot: Float4,
    offset_position: Float4,
    position: Float4,
    rotation: Rotor4,
    viewport_dimensions: Float3,
    horizontal: Float4,
    vertical: Float4,
    lower_left_corner: Float4,
}

impl Camera<Float4, Bivector4, Rotor4> for Camera4 
{
    fn calculate_camera_lower_left_corner(position: &Float4, rotation: &Rotor4, viewport_dimensions: &Float3) -> Float4
    {
        let centre_offset_dimensions = Float4::new(viewport_dimensions.x * 0.5,  viewport_dimensions.y * 0.5, viewport_dimensions.z, 0.0);
        let lower_left_corner: Float4 = *position - (*rotation * centre_offset_dimensions);
        return lower_left_corner;
    }

    fn recalculate(&mut self)
    {
        self.position = self.pivot + (self.rotation * self.offset_position);
        self.horizontal = self.rotation * Float4::new(self.viewport_dimensions.x, 0.0, 0.0, 0.0);
        self.vertical = self.rotation * Float4::new(0.0, self.viewport_dimensions.y, 0.0, 0.0);
        self.lower_left_corner = Self::calculate_camera_lower_left_corner(&self.position, &self.rotation, &self.viewport_dimensions);
    }

    fn new(pivot: Float4, offset_position: Float4, rotation: Rotor4, aspect_ratio: f32, vfov: f32, focal_length: f32) -> Self
    {
        let position: Float4 = pivot + (rotation * offset_position);

        let theta: f32 = mathematics::DEGREES_TO_RADIANS * vfov;
        let h = f32::tan(theta * 0.5);
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let viewport_dimensions: Float3 = Float3{x: viewport_width, y: viewport_height, z: focal_length};
        let horizontal: Float4 = rotation * Float4::new(viewport_width, 0.0, 0.0, 0.0);
        let vertical: Float4 = rotation * Float4::new(0.0, viewport_height, 0.0, 0.0);

        let lower_left_corner: Float4 = Self::calculate_camera_lower_left_corner(&position, &rotation, &viewport_dimensions);

        return Self{
            pivot: pivot,
            offset_position: offset_position,
            position: position,
            rotation: rotation,
            viewport_dimensions: viewport_dimensions,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
        };
    }

    fn get_camera_position(&self) -> Float4
    {
        return self.position;
    }

    fn get_ray_direction(&self, uv: Float2) -> Float4 
    {
        return self.lower_left_corner + uv.x * self.horizontal + uv.y * self.vertical - self.position;
    }

    fn set_camera_rotation(&mut self, rotation: Rotor4)
    {
        self.rotation = rotation;
        self.recalculate(); 
    }

    fn rotate_camera(&mut self, rotation: Rotor4)
    {
        self.rotation *= rotation;
        self.recalculate(); 
    }
}