use std::io;
use std::io::Write;

mod mathematics;
use mathematics::float3::Float3;
use mathematics::float2::Float2;

pub fn write_colour(out: &mut impl Write, pixel_colour: Float3) 
{
    // Write the translated [0, 255] value of each color component
    let r : i32 = (255.999 * pixel_colour.x) as i32;
    let g : i32 = (255.999 * pixel_colour.y) as i32;
    let b : i32 = (255.999 * pixel_colour.z) as i32;

    writeln!(out, "{} {} {}", r, g, b).expect("writing colour");
}


fn raymarch(ro: &Float3, rd: &Float3) -> Float3//f32
{
    let rdn: Float3 = rd.normalized();
    let t: f32 = 0.5 * (rdn.y + 1.0);
    return (1.0 - t) * Float3::new(1.0, 1.0, 1.0) + t * Float3::new(0.5, 0.7, 1.0)
}


fn main() 
{
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 480;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

    // Camera

    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = ASPECT_RATIO * viewport_height;
    let focal_length: f32 = 1.0;

    let origin = Float3::new(0.0, 0.0, -10.0);
    let lower_left_corner: Float3 = Float3::new(
        origin.x - viewport_width * 0.5,
        origin.y - viewport_height * 0.5,
        origin.z - focal_length
    ); 

    // Render

    let horizontal: Float3 = Float3::new(viewport_height, 0.0, 0.0); 
    let vertical: Float3 = Float3::new(0.0, viewport_width, 0.0); 

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
 
    for y in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", y);
        for x in 0..IMAGE_WIDTH {
            let uv: Float2 = Float2::new(
                x as f32 / (IMAGE_WIDTH - 1) as f32,
                y as f32 / (IMAGE_HEIGHT - 1) as f32
            );

            let direction: Float3 = lower_left_corner + uv.x * horizontal + uv.y * vertical - origin;

            //let distance: f32 = raymarch(&origin, &direction);

            let colour: Float3 = raymarch(&origin, &direction);
            write_colour(&mut io::stdout(), colour);
        }
    }
}
