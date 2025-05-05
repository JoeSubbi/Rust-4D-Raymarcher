pub struct Application
{
    pub aspect_ratio: f32,
    pub width: u32,
    pub height: u32,

    pub anti_aliasing: bool,
    pub render_4d: bool,

    pub max_frame_duration: f64,
}

impl Application
{
    pub fn new(aspect_ratio: f32, window_width: u32, anti_aliasing: bool, render_4d: bool, frame_rate_limit: i32) -> Application
    {
        return Application{
            aspect_ratio: aspect_ratio,
            width: window_width,
            height: (window_width as f32 / aspect_ratio) as u32,
            anti_aliasing: anti_aliasing,
            render_4d: render_4d,
            max_frame_duration: if frame_rate_limit > 0 { 1.0 / frame_rate_limit as f64 } else { -1.0 },
        };
    }
}