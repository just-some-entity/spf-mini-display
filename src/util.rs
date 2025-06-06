use num_traits::PrimInt;

pub trait DivisibleBy
{
    fn floor_to_nearest(self, value : usize) -> Self;
    fn ceil_to_nearest(self, value : usize) -> Self;
}

impl<T> DivisibleBy for T
where
    T: PrimInt,
{
    fn floor_to_nearest(self, value : usize) -> Self
    {
        self - (self % T::from(value).unwrap())
    }

    fn ceil_to_nearest(self, value : usize) -> Self
    {
        let rem = self % T::from(value).unwrap();
        if rem == T::zero() { self } else { self + (T::from(value).unwrap() - rem) }
    }
}