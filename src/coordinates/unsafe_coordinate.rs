//mod crate::coordinates::gen_coordinate;
//mod crate::gen_coordinate;
//use src::gen_coordinate;
//use crate::gen_coordinate::coordinate;

use super::gen_coordinate::MutableCoordinate;
use super::gen_coordinate::Modificable;

pub struct UnsafeCoordinate{
    x: f32,
    y: f32
}

impl MutableCoordinate<f32> for UnsafeCoordinate {
    fn new (x: f32, y: f32) -> UnsafeCoordinate{
    
        return UnsafeCoordinate{x:x,y:y};
        }
    fn get_x(&self) -> f32 { self.x }
    fn get_y(&self) -> f32 { self.y }
    fn set_x(&mut self, x: f32) -> () {self.x = x;}
    
    fn distancia(&self, otro: &UnsafeCoordinate) -> f32 {
        let difx = self.x - otro.x;
        let dify = self.y - otro.y;
        (difx.powi(2) + dify.powi(2)).sqrt()
    }
    fn negative(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }

    fn add(&mut self, altcoordinate: &Self) {
        self.x+=altcoordinate.x;
        self.y+=altcoordinate.y;
    }
    fn sub(&mut self, altcoordinate: &Self) -> () {
        self.x-=altcoordinate.x;
        self.y-=altcoordinate.y;
    }
    fn product(&mut self, altcoordinate: &Self) -> () {
        self.x*=altcoordinate.x;
        self.y*=altcoordinate.y;
    }
    fn equal(&mut self, altcoordinate: &Self) -> bool {
        return self.x == altcoordinate.x && self.y== altcoordinate.y;
    }
    fn c_mod(&mut self, altcoordinate: &Self) -> f32 {
        return self.x%altcoordinate.x + self.y%altcoordinate.y;
    }
    fn true_div(&mut self, altcoordinate: &Self) -> () {
        self.x*=altcoordinate.x;
        self.y*=-altcoordinate.y;
    }
   fn set_y(&mut self, y: f32) -> () {self.y = y;}



}

impl Modificable<f32> for UnsafeCoordinate {
    fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    fn set_y(&mut self, y: f32) {
        self.y = y;
    }
}
