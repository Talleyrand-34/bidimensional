
pub trait Coordinate<T>{
    fn new(x:T,y:T) -> Self;
    fn get_x(&self) -> T;
    fn get_y(&self) -> T;
    fn destroy(&self) ->(){
        drop(self);
    }
}

pub trait UnmutableCoordinate<T>{
    fn negative(&self) -> Self;    
    fn add(&self, altcoordinate: &Self) -> Self;
    fn sub(&self, altcoordinate: &Self) -> Self;
    fn product(&self, altcoordinate: &Self) -> Self;
    fn true_div(&self, altcoordinate: &Self) -> Self;
}
pub trait MutableCoordinate<T>{
    fn negative(&mut self) -> ();
    fn add(&mut self, altcoordinate: &Self) -> ();
    fn sub(&mut self, altcoordinate: &Self) -> ();
    fn product(&mut self, altcoordinate: &Self) -> ();
    fn true_div(&mut self, altcoordinate: &Self) -> ();
    fn set_x(&mut self, x: T) -> ();
    fn set_y(&mut self, y: T) -> ();
}

pub trait OpCoordinates<T>{
    fn equal(&self, altcoordinate: &Self) -> bool;
    fn c_mod(&self, altcoordinate: &Self) -> T;
    fn distancia(&self, altcoordinate: &Self) -> T;
}
