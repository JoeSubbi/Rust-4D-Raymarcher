use std::path::Path;
use std::time::{Instant, Duration};

use clap::Parser;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::rect::{Point, Rect};
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::image::LoadSurface;

mod application;
use application::Application;

mod camera;
use camera::Camera;

mod distance_functions;
use distance_functions::sdf_sphere;

mod mathematics;
use mathematics::float2::Float2;
use mathematics::float3::Float3;
use mathematics::float4::Float4;
use mathematics::multivectors::{Magnitude, Vector};


fn get_dist3(p: Float3) -> f32
{
    return sdf_sphere::<Float3>(p, Float3::new(0.0,0.0,0.0), 1.0);
}

fn get_dist4(p: Float4) -> f32
{
    return sdf_sphere::<Float4>(p, Float4::new(0.0,0.0,0.0, 0.0), 1.0);
}

const MAX_DIST: f32 = 100.0;
const MAX_STEPS: i32 = 100;
const SURF_DIST: f32 = 0.001;

fn raymarch<T: Vector>(ro: &T, rd: &T, distance_function: fn(T) -> f32) -> f32
{
    let mut d_origin: f32 = 0.0; // Distance from Origin

    for _i in 0..MAX_STEPS
    {
        let p: T = *ro + (*rd * d_origin);
        let d_surface: f32  = distance_function(p);
        d_origin += d_surface;
        if d_surface < SURF_DIST || d_origin > MAX_DIST
        {
            break;
        }
    }

    return d_origin;
}

fn normal3(p: Float3) -> Float3
{
    const E: f32 = 0.01;
    let n: Float3 = get_dist3(p) - Float3::new(
        get_dist3(p - Float3::new(E, 0.0, 0.0)),
        get_dist3(p - Float3::new(0.0, E, 0.0)),
        get_dist3(p - Float3::new(0.0, 0.0, E))
    );
    
    if n.length_squared() == 0.0
    {
        return n;
    }

    return n.normalized();
}

fn normal4(p: Float4) -> Float4
{
    const E: f32 = 0.01;
    let n: Float4 = get_dist4(p) - Float4::new(
        get_dist4(p - Float4::new(E, 0.0, 0.0, 0.0)),
        get_dist4(p - Float4::new(0.0, E, 0.0, 0.0)),
        get_dist4(p - Float4::new(0.0, 0.0, E, 0.0)),
        get_dist4(p - Float4::new(0.0, 0.0, 0.0, E))
    );
    
    if n.length_squared() == 0.0
    {
        return n;
    }

    return n.normalized();
}

fn get_pixel_colour(uv: &Float2, camera: &Camera, render_4d: bool) -> Float3
{
    let mut colour: Float3 = Float3::new(0.0, 0.0, 0.0);

    let direction: Float3 = camera.get_ray_direction(*uv);
    
    if !render_4d
    {
        let distance: f32 = raymarch(&camera.position, &direction, get_dist3);  
    
        if distance <= MAX_DIST
        {
            let p: Float3 = camera.position + (distance * direction);
            let n: Float3 = normal3(p);
            
            const LIGHT_SOURCE: Float3 = Float3{x: 2.0, y: 2.0, z: 4.0};

            let diffuse: f32 = Float3::dot(n, (LIGHT_SOURCE - p).normalized());
            colour = Float3::new(diffuse, diffuse, diffuse);
        }
    }
    else 
    {
        let position_4d = Float4::from(camera.position);
        let direction_4d = Float4::from(direction);
        let distance: f32 = raymarch(&position_4d, &direction_4d, get_dist4);  
    
        if distance <= MAX_DIST
        {
            let p: Float4 = position_4d + (distance * direction_4d);
            let n: Float4 = normal4(p);
            
            const LIGHT_SOURCE: Float4 = Float4{x: 2.0, y: 2.0, z: 4.0, w: 0.0};

            let diffuse: f32 = Float4::dot(n, (LIGHT_SOURCE - p).normalized());
            colour = Float3::new(diffuse, diffuse, diffuse);
        }
    }

    return colour;
}

fn update(delta_time: f64)
{

}


fn format_colour(pixel_colour: Float3) -> Color
{
    let draw_colour: Color = Color{
        r: (255f32 * pixel_colour.x) as u8,
        g: (255f32 * pixel_colour.y) as u8,
        b: (255f32 * pixel_colour.z) as u8,
        a: 1
    };

    return draw_colour;
}

fn render(canvas: &mut WindowCanvas, render_settings: &Application) -> Result<(), String>
{
    let camera: Camera = Camera::new(Float3::new(0.0, 0.0, 2.0), render_settings.aspect_ratio, 2.0, 1.0);
 
    for y in (0..render_settings.height).rev() {
        for x in 0..render_settings.width {
            let mut colour: Float3 = Float3::new(0.0, 0.0, 0.0);

            if !render_settings.anti_aliasing
            {
                let uv: Float2 = Float2::new(
                    x as f32 / (render_settings.width - 1) as f32,
                    y as f32 / (render_settings.height - 1) as f32
                );
                colour = get_pixel_colour(&uv, &camera, render_settings.render_4d);
            }
            else 
            {
                let sample_offsets: [Float2; 9] = [
                    Float2::new(0.5, 0.5),
                    Float2::new(0.0, 0.5),
                    Float2::new(-0.5, 0.5),
                    Float2::new(0.5, 0.0),
                    Float2::new(0.0, 0.0),
                    Float2::new(-0.5, 0.0),
                    Float2::new(0.5, -0.5),
                    Float2::new(0.0, -0.5),
                    Float2::new(-0.5, -0.5)
                ];
                for sample_offset in sample_offsets
                {
                    let uv: Float2 = Float2::new(
                        (x as f32 + sample_offset.x) / (render_settings.width - 1) as f32,
                        (y as f32 + sample_offset.y) / (render_settings.height - 1) as f32
                    );
                    colour += get_pixel_colour(&uv, &camera, render_settings.render_4d);
                }
                colour = colour / sample_offsets.len() as f32;
                
            }
            
            canvas.set_draw_color(format_colour(colour));
            canvas.draw_point(Point::new(x as i32, (render_settings.height as i32) - (y as i32)))?;
        }
    }

    Ok(())
}

#[derive(Parser, Debug)]
struct Args {
    // Use Anti Aliasing
    #[arg(long)]
    aa: bool,

    // Render 3D Scene
    #[arg(long)]
    d: bool
}

fn main() -> Result<(), String>
{
    let args: Args = Args::parse();
    let use_anti_aliasing: bool = args.aa;
    let render_4d: bool = !args.d;

    let application: Application = Application::new(16.0 / 9.0, 480, use_anti_aliasing, render_4d, 120);


    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Create Window
    let mut window = video_subsystem.window("4D Raymarching", application.width, application.height)
        .position_centered()
        .build()
        .unwrap();

    window.set_resizable(false);
    let window_icon = Surface::from_file("assets/ClientIcon.ico").unwrap();
    window.set_icon(window_icon);


    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // Prepare Fonts
    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path: &Path = Path::new(&"assets/CascadiaCode.ttf");
    let font: Font = ttf_context.load_font(font_path, 12).unwrap();

    let mut event_pump = sdl_context.event_pump()?;
    
    let mut fps_text: String;
    let mut fps_text_surface: Surface;
    let mut fps_text_texture: Texture;
    let mut fps_text_rect: Rect;

    let mut delta_duration: Duration = Duration::new(0, 0);
    'running: loop {
        
        let frame_start_time: Instant = Instant::now();
        
        // Handle Events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running;
                },
                _ => {}
            }
        }
        
        // Update any other logic
        let delta_time: f64 = delta_duration.as_secs_f64();
        if delta_time > 0.0
        {
            update(delta_time);
        }
        
        // Render
        canvas.clear();
        render(&mut canvas, &application)?;
        
        // Render FPS Text
        fps_text = format!("{:.1}fps {:.6}s", 1.0f64 / delta_time, delta_time);
        fps_text_surface = font.render(&fps_text).blended(Color::RGBA(255, 255, 255, 255)).unwrap();
        let width: u32 = fps_text_surface.width();
        let height: u32 = fps_text_surface.height();
        fps_text_texture = texture_creator.create_texture_from_surface(fps_text_surface).unwrap();
        fps_text_rect = Rect::new(application.width as i32 - width as i32,
                                        application.height as i32 - height as i32,
                                        width,
                                        height);
        canvas.copy(&fps_text_texture, None, Some(fps_text_rect))?;
        
        // Present full image
        canvas.present();
        
        delta_duration = frame_start_time.elapsed();
        
        // Handle frame rate caps
        if application.max_frame_duration > 0.0
        {
            if delta_duration.as_secs_f64() < application.max_frame_duration
            {
                let remaining_frame_time: f64 = application.max_frame_duration - delta_duration.as_secs_f64();
                ::std::thread::sleep(Duration::from_secs_f64(remaining_frame_time));
            }
        }
    }

    Ok(())
}
