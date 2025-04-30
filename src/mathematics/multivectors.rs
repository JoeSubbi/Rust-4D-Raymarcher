
#[allow(dead_code)]
pub trait Magnitude
{
    fn length(&self) -> f32
    {
        let length : f32 = self.length_squared();
        return f32::sqrt(length);
    }

    fn length_squared(&self) -> f32;

    fn normalize(&mut self);

    fn normalized(&self) -> Self;
}

pub trait Vector: 
    Copy +
    Magnitude +
    Sized + 
    std::ops::Add<Output = Self> +
    std::ops::AddAssign +
    std::ops::Neg<Output = Self> +
    std::ops::SubAssign +
    std::ops::Sub<Output = Self>
{
    fn dot(u: Self, v: Self) -> f32;
}

pub trait Bivector: 
    Copy
{

}

#[allow(dead_code)]
pub trait Rotor<V: Vector, B: Bivector>: 
    Copy +
    Magnitude +
    Sized + 
    std::ops::Mul<Output = Self> +
    std::ops::MulAssign
{
    /// Creates a new rotor in the specified bivector given an angle in radians
    fn bivector_angle(bv: &B, angle: f32) -> Self;

    fn geometric_product(u: V, v: V) -> Self;

    fn reverse(r: &Self) -> Self;

    fn reverse_me(&mut self);

    fn rotate_rotor(a: &Self, b: &Self) -> Self;

    fn rotate_vector(&self, v: V) -> V;

    fn slerp(from: &Self, to: &Self, ratio: f32) -> Self;

    /// Gives the angle of this Rotor in Radians
    fn angle(&self) -> f32;

    fn from_to(a: &Self, b: &Self) -> Self
    {
        let difference: Self = *b * Self::reverse(a);
        difference.normalized();
        return difference;
    }

    fn approx_equal(a: &Self, b: &Self) -> bool
    {
        let angle: f32 = Self::from_to(a, b).angle();
        return angle < 0.001;
    }
}