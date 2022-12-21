
//! CoordinateBasics Trait implements the basic logic over a coordinate and works like a common identifier
//! for the coordinates on external applications.
//!  This trait defines the basic logic over a unmutable version of a coordinate.
//! UnmutableCoordinate is to follow the unmutability principle to prevent error and matain consistency
//! on build up applications.
//! MutableCoordinate trait defines the basic logic over a mutable version of a coordinate.
//! This is for those applications that prefer speed over consistency.
//! 
pub trait CoordinateBasics<T>{
    /// create a new coordinate
    fn new(x:T,y:T) -> Self;
    /// get the x axis parameter
    fn get_x(&self) -> T;
    /// get y axis parameter
    fn get_y(&self) -> T;
    /// destroy the coordinate
    fn destroy(&self) ->(){
        drop(self);
    }
    
    
}


pub trait UnmutableCoordinate<T>{
    /// create a new unmutable coordinate which parameters are negative 
    fn negative(&self) -> Self;    
    /// create a new unmutable coordinate which parameters are the addition of the self and the other parameter
    fn add(&self, altcoordinate: &Self) -> Self;
    /// create a new unmutable coordinate which parameters are the substraction of the self and the other parameter
    fn sub(&self, altcoordinate: &Self) -> Self;
    /// create a new unmutable coordinate which parameters are the product of the self and the other parameter
    fn product(&self, altcoordinate: &Self) -> Self;
    /// create a new unmutable coordinate which parameters are the division of the self and the other parameter
    fn true_div(&self, altcoordinate: &Self) -> Self;
}

/// 
pub trait MutableCoordinate<T>{
    /// converts the coordinate into its negative representation
    fn negative(&mut self) -> ();
    /// adds the self coordinate the parameter coordinate
    fn add(&mut self, altcoordinate: &Self) -> ();
    /// substracts the self coordinate the parameter coordinate
    fn sub(&mut self, altcoordinate: &Self) -> ();
    /// add the self coordinate the parameter coordinate
    fn product(&mut self, altcoordinate: &Self) -> ();
    /// divides the self coordinate by the parameter altcoordinate
    fn true_div(&mut self, altcoordinate: &Self) -> ();
    /// Sets the first coordinate to x
    fn set_x(&mut self, x: T) -> ();
    /// Sets the second coordinate to y
    fn set_y(&mut self, y: T) -> ();
}


