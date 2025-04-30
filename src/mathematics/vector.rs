
pub trait Vector
{
    #[allow(dead_code)]
    fn length(&self) -> f32
    {
        let length : f32 = self.length_squared();
        return f32::sqrt(length);
    }

    fn length_squared(&self) -> f32;

    #[allow(dead_code)]
    fn normalize(&mut self);

    #[allow(dead_code)]
    fn normalized(&self) -> Self;
}

pub trait VectorGrade1: Vector +
    Sized + 
    std::ops::Add<Output = Self> +
    std::ops::AddAssign +
    std::ops::Neg<Output = Self> +
    std::ops::SubAssign +
    std::ops::Sub<Output = Self>
{

}