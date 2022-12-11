//mod crate::coordinates::gen_coordinate;
//mod crate::gen_coordinate;
//use src::gen_coordinate;
//use crate::gen_coordinate::coordinate;

use super::gen_coordinate::Coordinate;
use super::gen_coordinate::MutableCoordinate;
use super::gen_coordinate::OpCoordinates;

pub struct UnsafeCoordinate{
    x: f32,
    y: f32
}
impl Coordinate<f32> for UnsafeCoordinate {
    fn new (x: f32, y: f32) -> UnsafeCoordinate{return UnsafeCoordinate{x:x,y:y};}
    fn get_x(&self) -> f32 { self.x }
    fn get_y(&self) -> f32 { self.y }
    fn destroy(&self) ->() {drop(self)}

}



impl MutableCoordinate<f32> for UnsafeCoordinate {
    fn set_x(&mut self, x: f32) -> () {self.x = x;}
    fn set_y(&mut self, y: f32) -> () {self.y = y;}
    
    fn negative(&mut self)-> (){
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
    
    fn true_div(&mut self, altcoordinate: &Self) -> () {
        self.x*=altcoordinate.x;
        self.y*=-altcoordinate.y;
    }
}


impl OpCoordinates<f32> for UnsafeCoordinate{
    fn distancia(&self, otro: &UnsafeCoordinate) -> f32 {
        let difx:f32 = self.x - otro.x;
        let dify:f32= self.y - otro.y;
        (difx.powi(2) + dify.powi(2)).sqrt()
    }
    fn equal(&self, altcoordinate: &Self) -> bool {
        return self.x == altcoordinate.x && self.y== altcoordinate.y;
    }
    fn equiv(&self, altcoordinate: &Self) -> bool {
        let x:f32;
        let y:f32;
        if altcoordinate.x!=0.0 {
            x = self.x/altcoordinate.x;
        }
        else if self.x!=0.0 {
            x = altcoordinate.x/self.x;
        }
        else {
            x=0.0;
        }
        if altcoordinate.y!=0.0 {
            y= self.y/altcoordinate.y;
        }
        else if self.y!=0.0 {
            y = altcoordinate.y/self.y;
        }
        else {
            y=0.0;
         }

        return x==y;
    }
}
