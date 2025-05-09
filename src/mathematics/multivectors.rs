
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
    std::ops::Sub<Output = Self> +
    std::ops::Mul<f32, Output = Self> + 
    std::ops::MulAssign<f32> + 
    std::ops::Div<f32, Output = Self>
{
    fn dot(u: Self, v: Self) -> f32;
}

pub trait Bivector: 
    Copy +
    Magnitude
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
    /// Creates a new Rotor in the specified bivector given an angle in radians
    fn bivector_angle(bv: &B, angle: f32) -> Self;

    /// Creates a Rotor that is double the rotation from Vector v to Vector u
    fn geometric_product(u: V, v: V) -> Self;

    /// Return a copy of the reverse of this Rotor
    fn reverse(r: &Self) -> Self;

    /// Reverse this Rotor
    fn reverse_me(&mut self);

    fn rotate_rotor(a: &Self, b: &Self) -> Self;

    fn rotate_vector(&self, v: V) -> V;

    /// Spherical Linear Interpolation from a Rotor to another
    fn slerp(from: &Self, to: &Self, ratio: f32) -> Self;

    /// Gives the angle of this Rotor in Radians
    fn angle(&self) -> f32;

    /// Returns the Rotor representing the rotation from Rotor a to Rotor b
    fn from_to(a: &Self, b: &Self) -> Self
    {
        let difference: Self = *b * Self::reverse(a);
        difference.normalized();
        return difference;
    }

    /// Check if the the angle between Rotor a and Rotor b is < 0.001 radians or 0.057 degrees
    fn approx_equal(a: &Self, b: &Self) -> bool
    {
        let angle: f32 = Self::from_to(a, b).angle();
        return angle < 0.001;
    }
}