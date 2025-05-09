pub struct Application
{
    pub aspect_ratio: f32,
    pub width: u32,
    pub height: u32,

    pub anti_aliasing: bool,
}

impl Application
{
    pub fn new(aspect_ratio: f32, window_width: u32, anti_aliasing: bool) -> Application
    {
        return Application{
            aspect_ratio: aspect_ratio,
            width: window_width,
            height: (window_width as f32 / aspect_ratio) as u32,
            anti_aliasing: anti_aliasing,
        };
    }
}