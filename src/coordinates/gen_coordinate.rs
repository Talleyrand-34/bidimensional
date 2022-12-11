
/// This Trait implements the basic logic over a coordinate and works like a common identifier
/// for the coordinates on external applications.
pub trait Coordinate<T>{
    fn new(x:T,y:T) -> Self;
    fn get_x(&self) -> T;
    fn get_y(&self) -> T;
    fn destroy(&self) ->(){
        drop(self);
    }
}
/// This trait defines the basic logic over a unmutable version of a coordinate.
/// This is to follow the unmutability principle to prevent error and matain consistency
/// on build up applications.
pub trait UnmutableCoordinate<T>{
    fn negative(&self) -> Self;    
    fn add(&self, altcoordinate: &Self) -> Self;
    fn sub(&self, altcoordinate: &Self) -> Self;
    fn product(&self, altcoordinate: &Self) -> Self;
    fn true_div(&self, altcoordinate: &Self) -> Self;
}

/// This trait defines the basic logic over a mutable version of a coordinate.
/// This is for those applications that prefer speed over consistency.
pub trait MutableCoordinate<T>{
    fn negative(&mut self) -> ();
    fn add(&mut self, altcoordinate: &Self) -> ();
    fn sub(&mut self, altcoordinate: &Self) -> ();
    fn product(&mut self, altcoordinate: &Self) -> ();
    fn true_div(&mut self, altcoordinate: &Self) -> ();
    fn set_x(&mut self, x: T) -> ();
    fn set_y(&mut self, y: T) -> ();
}

/// This trait contains the operations that given to coordinates return a primitive type.
pub trait OpCoordinates<T>{
    fn equal(&self, altcoordinate: &Self) -> bool;
    fn equiv(&self, altcoordinate: &Self) -> bool;
    fn distancia(&self, altcoordinate: &Self) -> T;
}
