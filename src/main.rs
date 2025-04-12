use std::io;
use std::io::Write;
use clap::Parser;

mod camera;
use camera::Camera;

mod distance_functions;
use distance_functions::sdf_sphere;

mod mathematics;
use mathematics::float3::Float3;
use mathematics::float2::Float2;

fn format_colour(pixel_colour: Float3) -> String
{
    let r : i32 = (255.999 * pixel_colour.x) as i32;
    let g : i32 = (255.999 * pixel_colour.y) as i32;
    let b : i32 = (255.999 * pixel_colour.z) as i32;

    let pixel_colour: String = format!("\n{} {} {}", r, g, b);
    return pixel_colour;
}

fn get_dist(p: Float3) -> f32
{
    return sdf_sphere(p, Float3::new(0.0,0.0,0.0), 1.0);
}

const MAX_DIST: f32 = 100.0;
const MAX_STEPS: i32 = 100;
const SURF_DIST: f32 = 0.001;

fn raymarch(ro: &Float3, rd: &Float3) -> f32
{
    let mut d_origin: f32 = 0.0; // Distance from Origin

    for _i in 0..MAX_STEPS
    {
        let p: Float3 = *ro + (*rd * d_origin);
        let d_surface: f32  = get_dist(p);
        d_origin += d_surface;
        if d_surface < SURF_DIST || d_origin > MAX_DIST
        {
            break;
        }
    }

    return d_origin;
}

const LIGHT_SOURCE: Float3 = Float3{x: 2.0, y: 2.0, z: 4.0};

fn normal(p: Float3) -> Float3
{
    let e: f32 = 0.01;
    let n: Float3 = get_dist(p) - Float3::new(
        get_dist(p - Float3::new(e, 0.0, 0.0)),
        get_dist(p - Float3::new(0.0, e, 0.0)),
        get_dist(p - Float3::new(0.0, 0.0, e))
    );
    
    if n.length_squared() == 0.0
    {
        return n;
    }

    return n.normalized();
}

fn get_pixel_colour(uv: &Float2, camera: &Camera) -> Float3
{
    let mut colour: Float3 = Float3::new(0.0, 0.0, 0.0);

    let direction: Float3 = camera.get_ray_direction(*uv);
    let distance: f32 = raymarch(&camera.position, &direction);

    if distance <= MAX_DIST
    {
        let p: Float3 = camera.position + (distance * direction);
        let n: Float3 = normal(p);
        let diffuse: f32 = Float3::dot(n, (LIGHT_SOURCE - p).normalized());
        colour = Float3::new(diffuse, diffuse, diffuse);
    }

    return colour;
}

#[derive(Parser, Debug)]
struct Args {
    // Use anti aliasing
    #[arg(long)]
    aa: bool,
}

fn main() 
{
    let args: Args = Args::parse();
    let use_anti_aliasing: bool = args.aa;

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 480;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

    let camera: Camera = Camera::new(Float3::new(0.0, 0.0, 2.0), ASPECT_RATIO, 2.0, 1.0);

    // Render

    let mut image_ppm: String = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
 
    for y in (0..IMAGE_HEIGHT).rev() {
        for x in 0..IMAGE_WIDTH {
            let mut colour: Float3 = Float3::new(0.0, 0.0, 0.0);

            if !use_anti_aliasing
            {
                let uv: Float2 = Float2::new(
                    x as f32 / (IMAGE_WIDTH - 1) as f32,
                    y as f32 / (IMAGE_HEIGHT - 1) as f32
                );
                colour = get_pixel_colour(&uv, &camera);
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
                        (x as f32 + sample_offset.x) / (IMAGE_WIDTH - 1) as f32,
                        (y as f32 + sample_offset.y) / (IMAGE_HEIGHT - 1) as f32
                    );
                    colour += get_pixel_colour(&uv, &camera);
                }
                colour = colour / sample_offsets.len() as f32;
                
            }
            
            image_ppm.push_str(format_colour(colour).as_str());
        }
    }

    writeln!(&mut io::stdout(), "{}",image_ppm.as_str()).expect("writing colour");
}
