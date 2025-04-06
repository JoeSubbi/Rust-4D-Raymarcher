mod mathematics;
use mathematics::float3::Float3;

fn main() 
{
    let mut test : Float3 = Float3::new(0.0, 1.0, 0.0);
    test += test;
    test.normalize();

    println!("Test: {}", test);
}
