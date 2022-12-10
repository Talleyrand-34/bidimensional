pub trait UnmutableCoordinate<T>{
    fn new(x:T,y:T) -> Self;
    fn get_x(&self) -> T;
    fn get_y(&self) -> T;
    fn negative(&self) -> Self;
    fn distancia(&self, sec_Coordinate: &Self) -> T;
    fn add(&self, sec_Coordinate: &Self) -> Self;
    fn sub(&self, sec_Coordinate: &Self) -> Self;
    fn product(&self, sec_Coordinate: &Self) -> Self;
    fn true_div(&self, sec_Coordinate: &Self) -> Self;
    fn equal(&self, sec_Coordinate: &Self) -> bool;
    fn c_mod(&self, sec_Coordinate: &Self) -> Self;
}

pub trait MutableCoordinate<T>{
    fn new(x:T,y:T) -> Self;
    fn get_x(&self) -> T;
    fn get_y(&self) -> T;
    fn negative(&mut self) -> Self;
    fn distancia(&mut self, sec_Coordinate: &Self) -> Self;
    fn add(&mut self, sec_Coordinate: &Self) -> Self;
    fn sub(&mut self, sec_Coordinate: &Self) -> Self;
    fn product(&mut self, sec_Coordinate: &Self) -> Self;
    fn true_div(&mut self, sec_Coordinate: &Self) -> Self;
    fn equal(&mut self, sec_Coordinate: &Self) -> bool;
    fn c_mod(&mut self, sec_Coordinate: &Self) -> T;
    fn set_x(&mut self, x: T) -> Self;
    fn set_y(&mut self, y: T) -> Self;

}

pub trait Coordinate<T>{
    fn new(x:T,y:T) -> Self;
    fn get_x(&self) -> T;
    fn get_y(&self) -> T;
    fn negative(&self) -> Self;
    fn distancia(&self, sec_Coordinate: &Self) -> T;
    fn add(&self, sec_Coordinate: &Self) -> Self;
    fn sub(&self, sec_Coordinate: &Self) -> Self;
    fn product(&self, sec_Coordinate: &Self) -> Self;
    fn true_div(&self, sec_Coordinate: &Self) -> Self;
    fn equal(&self, sec_Coordinate: &Self) -> bool;
    fn c_mod(&self, sec_Coordinate: &Self) -> Self;
}
pub trait Modificable<T> {

    fn set_x(&mut self, x: T) -> ();
    fn set_y(&mut self, y: T) -> ();

}