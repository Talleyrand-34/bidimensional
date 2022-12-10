pub trait UnmutableCoordinate<T>{
    fn new(x:T,y:T) -> Self;
    fn get_x(&self) -> T;
    fn get_y(&self) -> T;
    fn negative(&self) -> Self;
    fn distancia(&self, altcoordinate: &Self) -> T;
    fn add(&self, altcoordinate: &Self) -> Self;
    fn sub(&self, altcoordinate: &Self) -> Self;
    fn product(&self, altcoordinate: &Self) -> Self;
    fn true_div(&self, altcoordinate: &Self) -> Self;
    fn equal(&self, altcoordinate: &Self) -> bool;
    fn c_mod(&self, altcoordinate: &Self) -> Self;
}

pub trait MutableCoordinate<T>{
    fn new(x:T,y:T) -> Self;
    fn get_x(&self) -> T;
    fn get_y(&self) -> T;
    fn distancia(&self, altcoordinate: &Self) -> T;
    fn negative(&mut self) -> ();
    fn add(&mut self, altcoordinate: &Self) -> ();
    fn sub(&mut self, altcoordinate: &Self) -> ();
    fn product(&mut self, altcoordinate: &Self) -> ();
    fn true_div(&mut self, altcoordinate: &Self) -> ();
    fn equal(&mut self, altcoordinate: &Self) -> bool;
    fn c_mod(&mut self, altcoordinate: &Self) -> T;
    fn set_x(&mut self, x: T) -> ();
    fn set_y(&mut self, y: T) -> ();

}

pub trait Coordinate<T>{
    fn new(x:T,y:T) -> Self;
    fn get_x(&self) -> T;
    fn get_y(&self) -> T;
    fn negative(&self) -> Self;
    fn distancia(&self, altcoordinate: &Self) -> T;
    fn add(&self, altcoordinate: &Self) -> Self;
    fn sub(&self, altcoordinate: &Self) -> Self;
    fn product(&self, altcoordinate: &Self) -> Self;
    fn true_div(&self, altcoordinate: &Self) -> Self;
    fn equal(&self, altcoordinate: &Self) -> bool;
    fn c_mod(&self, altcoordinate: &Self) -> T;
}
pub trait Modificable<T> {

    fn set_x(&mut self, x: T) -> ();
    fn set_y(&mut self, y: T) -> ();

}