use std::marker::PhantomData;
use super::gen_coordinate::UnmutableCoordinate;
pub struct UnmutPhantomCoordinate<T:Copy>{
    x:T,
    y:T,
    marker: PhantomData<T>,
}

impl<T> UnmutPhantomCoordinate<T> {
    fn new(x: T, y: T) -> UnmutPhantomCoordinate<T> {
        UnmutPhantomCoordinate { x, y, marker: PhantomData }
    }    
}

impl<T> UnmutableCoordinate<T> for UnmutPhantomCoordinate<T> {
    /// create a new unmutable coordinate which parameters are negative 
    fn negative(&self) -> Self{
        UnmutPhantomCoordinate::new(-self.x, -self.y);
    }    
    /// create a new unmutable coordinate which parameters are the addition of the self and the other parameter
    fn add(&self, altcoordinate: &Self) -> Self;
    /// create a new unmutable coordinate which parameters are the substraction of the self and the other parameter
    fn sub(&self, altcoordinate: &Self) -> Self;
    /// create a new unmutable coordinate which parameters are the product of the self and the other parameter
    fn product(&self, altcoordinate: &Self) -> Self;
    /// create a new unmutable coordinate which parameters are the division of the self and the other parameter
    fn true_div(&self, altcoordinate: &Self) -> Self;
}
