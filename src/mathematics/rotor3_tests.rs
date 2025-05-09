use crate::mathematics::approx_equal;
use crate::mathematics::bivector3::Bivector3;
use crate::mathematics::float3::Float3;
use crate::mathematics::PI;
use crate::mathematics::rotor3::Rotor3;
use crate::mathematics::multivectors::{Magnitude, Rotor};

fn rotate_vector(v: &mut Float3, e1: Float3, e2: Float3, angle: f32)
{
    let bv: Bivector3 = Float3::wedge(e1, e2);
    let r: Rotor3 = Rotor3::bivector_angle(&bv, angle);

    *v = r * *v;
}

#[test]
fn geometric_product()
{
    // Rotors have double coverage, so the rotation will actually be twice the rotation from e1 to e2
    let e1: Float3 = Float3::new(1.0, 0.0, 0.0);
    let e2: Float3 = Float3::new(1.0, 1.0, 0.0).normalized();

    let geo_prod: Rotor3 = Rotor::geometric_product(e2, e1);

    let v: Float3 = e1;
    let expected: Float3 = Float3::new(0.0, 1.0, 0.0);

    assert_eq!(geo_prod * v, expected);
}

#[test]
fn single_planar_rotations()
{
    let mut v: Float3;
    let mut expected: Float3;
    
    let mut e1: Float3;
    let mut e2: Float3;

    let a: f32 = PI / 2.0;

    // XY Rotation
    e1 = Float3::new(1.0, 0.0, 0.0);
    e2 = Float3::new(0.0, 1.0, 0.0);

    v = e1;
    expected = e2;

    rotate_vector(&mut v, e1, e2, a);
    assert_eq!(v, expected);
    
    // XZ Rotation
    e1 = Float3::new(1.0, 0.0, 0.0);
    e2 = Float3::new(0.0, 0.0, 1.0);

    v = e1;
    expected = e2;

    rotate_vector(&mut v, e1, e2, a);
    assert_eq!(v, expected);

    // YZ Rotation
    e1 = Float3::new(0.0, 1.0, 0.0);
    e2 = Float3::new(0.0, 0.0, 1.0);

    v = e1;
    expected = e2;

    rotate_vector(&mut v, e1, e2, a);
    assert_eq!(v, expected);
}

#[test]
fn zero_rotation()
{
    let a: f32 = PI / 2.0;

    // XY Rotation
    let e1: Float3 = Float3::new(1.0, 0.0, 0.0);
    let e2: Float3 = e1;

    let mut v: Float3 = Float3::new(1.0,1.0,1.0);
    let expected: Float3 = v;

    rotate_vector(&mut v, e1, e2, a);
    assert_eq!(v, expected);
}

#[test]
fn reverse()
{
    let e1 = Float3::new(1.0, 0.0, 0.0);
    let e2 = Float3::new(0.0, 1.0, 0.0);

    let r: Rotor3 = Rotor3::geometric_product(e1, e2);

    let v1: Float3 = e1;
    let mut v2: Float3 = r * v1;
    
    assert_ne!(v1, v2);
    
    let r_reverse: Rotor3 = Rotor3::reverse(&r);

    assert_eq!(r * r_reverse, Rotor3::IDENTITY);

    v2 = r_reverse * v2;
    
    assert_eq!(v1, v2);
}

#[test]
fn compounded_rotation()
{
    let mut v: Float3;
    let mut expected: Float3;
    
    let mut e1: Float3;
    let mut e2: Float3;

    let a: f32 = PI / 2.0;

    let mut bv: Bivector3;
    let mut r: Rotor3;

    // Single rotation off axes
    e1 = Float3::new(0.0, 0.0, 1.0);
    e2 = Float3::new(1.0, 1.0, 0.0).normalized();

    bv = Float3::wedge(e1, e2);
    r = Rotor3::bivector_angle(&bv, a);

    v = e1;
    expected = e2;

    v = r * v;
    assert_eq!(v, expected);

    // Two Combined Rotations
    v = Float3::new(0.0, 0.0, 1.0);
    r = Rotor3::IDENTITY;

    e1 = Float3::new(0.0, 0.0, 1.0);
    e2 = Float3::new(0.0, 1.0, 0.0);

    bv = Float3::wedge(e1, e2);
    r *=Rotor3::bivector_angle(&bv, a);

    e1 = Float3::new(0.0, 0.0, 1.0);
    e2 = Float3::new(1.0, 0.0, 0.0);

    bv = Float3::wedge(e1, e2);
    r *=Rotor3::bivector_angle(&bv, a);

    expected = Float3::new(1.0, 0.0, 0.0);

    v = r * v;
    assert_eq!(v, expected);
}

#[test]
fn single_rotation_angle()
{
    let mut expected_angle: f32;
    let mut actual_angle: f32;

    let mut e1: Float3;
    let mut e2: Float3;

    let mut bv: Bivector3;
    let mut r: Rotor3;

    // 90 Degrees (PI/2)
    expected_angle = PI / 2.0;

    e1 = Float3::new(1.0, 0.0, 0.0);
    e2 = Float3::new(0.0, 1.0, 0.0);
    bv = Float3::wedge(e1, e2);

    r = Rotor3::bivector_angle(&bv, expected_angle);
    actual_angle = r.angle();

    assert!(approx_equal(expected_angle, actual_angle));

    // 45 Degrees (PI/4)
    expected_angle = PI / 4.0;

    e1 = Float3::new(0.0, 1.0, 0.0);
    e2 = Float3::new(0.0, 0.0, 1.0);
    bv = Float3::wedge(e1, e2);

    r  = Rotor3::bivector_angle(&bv, expected_angle);
    actual_angle = r.angle();

    assert!(approx_equal(expected_angle, actual_angle));

    // 180 Degrees (PI)
    expected_angle = PI;

    e1 = Float3::new(1.0, 0.0, 0.0);
    e2 = Float3::new(0.0, 0.0, 1.0);
    bv = Float3::wedge(e1, e2);

    r = Rotor3::bivector_angle(&bv, expected_angle);
    actual_angle = r.angle();

    assert!(approx_equal(expected_angle, actual_angle));

    // 360 / 0 Degrees (2PI)
    expected_angle = 0.0;

    e1 = Float3::new(0.0, 1.0, 0.0);
    e2 = Float3::new(0.0, 0.0, 1.0);
    bv = Float3::wedge(e1, e2);

    r = Rotor3::bivector_angle(&bv, expected_angle);
    actual_angle = r.angle();

    assert!(approx_equal(expected_angle, actual_angle));
}

#[test]
fn combined_rotation_angle()
{
    let a: f32 = PI / 2.0;
    let mut expected_angle: f32;
    let mut actual_angle: f32;

    let mut e1: Float3;
    let mut e2: Float3;

    let mut bv: Bivector3;
    let mut r: Rotor3;
    
    // Single rotation off axes

    e1 = Float3::new(0.0, 0.0, 1.0);
    e2 = Float3::new(1.0, 1.0, 0.0).normalized();

    bv = Float3::wedge(e1, e2);

    expected_angle = a;
    r = Rotor3::bivector_angle(&bv, a);
    actual_angle = r.angle();

    assert!(approx_equal(expected_angle, actual_angle));

    // Two Combined Rotations
    r = Rotor3::IDENTITY;

    e1 = Float3::new(0.0, 0.0, 1.0);
    e2 = Float3::new(0.0, 1.0, 0.0);

    bv = Float3::wedge(e1, e2);
    r *= Rotor3::bivector_angle(&bv, a);

    e1 = Float3::new(0.0, 0.0, 1.0);
    e2 = Float3::new(1.0, 0.0, 0.0);

    bv = Float3::wedge(e1, e2);
    r *= Rotor3::bivector_angle(&bv, a);

    expected_angle = a;
    r = Rotor3::bivector_angle(&bv, a);
    actual_angle = r.angle();

    assert!(approx_equal(expected_angle, actual_angle));
}

#[test]
fn slerp()
{
    let mut r_slerp: Rotor3;

    let e1: Float3 = Float3::new(1.0, 0.0, 0.0);
    let e2: Float3 = Float3::new(0.0, 0.0, 1.0);
    
    let r_a: Rotor3 = Rotor3::IDENTITY;
    
    let bv: Bivector3 = Float3::wedge(e1, e2);
    let r_b: Rotor3 = Rotor3::bivector_angle(&bv, PI / 2.0);

    // Check a vector, v, when rotated from x to z is half way between at t = 0.5

    r_slerp = Rotor3::slerp(&r_a, &r_b, 0.5);

    let v: Float3 = Float3::new(1.0, 0.0, 0.0);
    let expected: Float3 = Float3::new(1.0, 0.0, 1.0).normalized();

    assert_eq!(r_slerp * v, expected);

    // Define two rotors, check t = 0 and t = 1 produce either rotor 

    r_slerp = Rotor3::slerp(&r_a, &r_b, 0.0);

    assert_eq!(r_slerp, r_a);

    r_slerp = Rotor3::slerp(&r_a, &r_b, 1.0);

    assert_eq!(r_slerp, r_b);
}