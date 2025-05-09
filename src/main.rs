use std::path::Path;
use std::thread;
use std::time::Instant;
use std::sync::Mutex;

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
use camera::*;

mod distance_functions;
use distance_functions::*;

mod mathematics;
use mathematics::bivector3::Bivector3;
use mathematics::bivector4::Bivector4;
use mathematics::float2::Float2;
use mathematics::float3::Float3;
use mathematics::float4::Float4;
use mathematics::multivectors::{Magnitude, Rotor, Vector};
use mathematics::rotor3::Rotor3;
use mathematics::rotor4::Rotor4;


fn get_dist3(p: Float3) -> f32
{
    //return sdf_sphere::<Float3>(p, Float3::new(0.0,0.0,0.0), 1.5);
    return sdf_box3(p, Float3::new(0.0,0.0,0.0), Float3::new(1.0,1.0,1.0), 0.01)
}

fn get_dist4(p: Float4) -> f32
{
    //return sdf_sphere::<Float4>(p, Float4::new(0.0,0.0,0.0, 0.0), 1.5);
    return sdf_box4(p, Float4::new(0.0,0.0,0.0, 0.0), Float4::new(1.0,1.0,1.0, 1.0), 0.01)
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

fn get_pixel_colour(uv: &Float2, scene: &Scene) -> Float3
{
    let mut colour: Float3 = Float3::new(0.0, 0.0, 0.0);


    if !scene.is4d
    {
        let scene_3d: &SubScene3 = scene.scene_3d.as_ref().expect("Trying to render unassigned scene"); 
        let ro: Float3 = scene_3d.camera.get_camera_position();
        let rd: Float3 = scene_3d.camera.get_ray_direction(*uv);

        let distance: f32 = raymarch(&ro, &rd, get_dist3);  
    
        if distance <= MAX_DIST
        {
            let p: Float3 = ro + (distance * rd);
            let n: Float3 = normal3(p);
            
            let diffuse: f32 = Float3::dot(n, (scene_3d.light_source - p).normalized()) * 0.5 + 0.5;
            colour = Float3::new(diffuse, diffuse, diffuse);
        }
    }
    else 
    {
        let scene_4d: &SubScene4 = scene.scene_4d.as_ref().expect("Trying to render unassigned scene"); 
        let ro: Float4 = scene_4d.camera.get_camera_position();
        let rd: Float4 = scene_4d.camera.get_ray_direction(*uv);

        let distance: f32 = raymarch(&ro, &rd, get_dist4);  
    
        if distance <= MAX_DIST
        {
            let p: Float4 = ro + (distance * rd);
            let n: Float4 = normal4(p);
            
            let diffuse: f32 = Float4::dot(n, (scene_4d.light_source - p).normalized()) * 0.5 + 0.5;
            colour = Float3::new(diffuse, diffuse, diffuse);
        }
    }

    return colour;
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

#[derive(Copy, Clone)]
struct Pixel
{
    pub colour: Color,
    pub x: u32,
    pub y: u32,
}

fn render_pixel(x: u32, y: u32, scene: &Scene, application: &Application) -> Pixel
{
    let mut colour: Float3 = Float3::new(0.0, 0.0, 0.0);
    
    if !application.anti_aliasing
    {
        let uv: Float2 = Float2::new(
            x as f32 / (application.width - 1) as f32,
            y as f32 / (application.height - 1) as f32
        );
        colour = get_pixel_colour(&uv, &scene);
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
                (x as f32 + sample_offset.x) / (application.width - 1) as f32,
                (y as f32 + sample_offset.y) / (application.height - 1) as f32
            );
            colour += get_pixel_colour(&uv, &scene);
        }
        colour = colour / sample_offsets.len() as f32;
    }

    return Pixel{colour: format_colour(colour), x: x, y: y};
}

fn render(canvas: &mut WindowCanvas, scene: &Scene, application: &Application)
{
    thread::scope(|s| {
        
        // Render using a new thread for each row of the image
        // A thread for each pixel was had drastically more overhead. A single thread per core using thread::available_parallelism() was not enough
        
        let mut pixel_threads = Vec::with_capacity(application.height as usize);

        for y in 0..application.height
        {
            pixel_threads.push(
                s.spawn(move || 
                    {
                        let mut pixel_row = Vec::with_capacity(application.width as usize);

                        for x in 0..application.width
                        {
                            pixel_row.push(render_pixel(x, y, &scene, &application));
                        }

                        return pixel_row;
                    }
                )
            );
        }

        for thread_handle in pixel_threads
        {
            for pixel in thread_handle.join().unwrap()
            {
                canvas.set_draw_color(pixel.colour);
                canvas.draw_point(Point::new(pixel.x as i32, pixel.y as i32)).unwrap();
            }
        }
    });
}

fn update(delta_time: f64, scene: &mut Scene)
{
    if scene.is4d
    {
        let r: Rotor4 = Rotor4::bivector_angle(&Bivector4::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0), delta_time as f32);

        let scene_4d: &mut Box<SubScene4> = scene.scene_4d.as_mut().expect("Unable to access 4D scene when 4D scene is specified"); 
        scene_4d.camera.rotate_camera(r);
        scene_4d.light_source = r * scene_4d.light_source;
    }
    else 
    {
        let r: Rotor3 = Rotor3::bivector_angle(&Bivector3::new(1.0, 1.0, 1.0), delta_time as f32);

        let scene_3d: &mut Box<SubScene3> = scene.scene_3d.as_mut().expect("Trying to render unassigned scene"); 
        scene_3d.camera.rotate_camera(r);
        scene_3d.light_source = r * scene_3d.light_source;
    }
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

struct Time
{
    pub application_up_time: f64,
    pub frame_delta_time: f64,
}

static APPLICATION_TIME: Mutex<Time> = Mutex::new(Time{application_up_time: 0.0, frame_delta_time: 0.0});

struct Scene
{
    pub is4d: bool,
    pub scene_3d: Option<Box<SubScene3>>,
    pub scene_4d: Option<Box<SubScene4>>,
}

struct SubScene3
{
    pub camera: Camera3,
    pub light_source: Float3,
}

struct SubScene4
{
    pub camera: Camera4,
    pub light_source: Float4,
}

fn main() -> Result<(), String>
{
    let args: Args = Args::parse();
    let use_anti_aliasing: bool = args.aa;
    let render_4d: bool = !args.d;

    let application: Application = Application::new(16.0 / 9.0, 480, use_anti_aliasing);

    let mut scene = Scene{
        is4d: render_4d,
        scene_3d: if !render_4d {
            Some(Box::new(SubScene3 
                { 
                    camera: Camera3::new(Float3::new(0.0, 0.0, 0.0), Float3::new(0.0, 0.0, 4.0), Rotor3::IDENTITY, application.aspect_ratio, 60.0, 1.0), 
                    light_source: Float3::new(2.0, 2.0, 4.0)
                }))
        } else { None },
        scene_4d: if render_4d {
            Some(Box::new(SubScene4 
                { 
                    camera: Camera4::new(Float4::new(0.0, 0.0, 0.0, 0.0), Float4::new(0.0, 0.0, 4.0, 0.0), Rotor4::IDENTITY, application.aspect_ratio, 60.0, 1.0), 
                    light_source: Float4::new(2.0, 2.0, 4.0, 0.0)
                }))
        } else { None },
    };

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

    let applciation_start_time: Instant = Instant::now();
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
        let delta_time: f64 = APPLICATION_TIME.lock().unwrap().frame_delta_time;
        update(delta_time, &mut scene);
        
        // Render
        canvas.clear();
        render(&mut canvas, &scene, &application);
        
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
        
        // Update time
        let mut time = APPLICATION_TIME.lock().unwrap();
        time.application_up_time = applciation_start_time.elapsed().as_secs_f64();
        time.frame_delta_time = frame_start_time.elapsed().as_secs_f64();
    }

    Ok(())
}
