//mod crate::coordinates::gen_coordinate;
//mod crate::gen_coordinate;
//use src::gen_coordinate;
//use crate::gen_coordinate::coordinate;

use super::gen_coordinate::Coordinate;
use super::gen_coordinate::Modificable;

pub struct UnsafeCoordinate{
    x: f32,
    y: f32
}

impl Coordinate<f32> for UnsafeCoordinate {
    fn new (x: f32, y: f32) -> UnsafeCoordinate{
    
        return UnsafeCoordinate{x:x,y:y};
        }
    fn get_x(&self) -> f32 { self.x }
    fn get_y(&self) -> f32 { self.y }
    fn distancia(&self, otro: &UnsafeCoordinate) -> f32 {
        let difx = self.x - otro.x;
        let dify = self.y - otro.y;
        (difx.powi(2) + dify.powi(2)).sqrt()
    }
   



}

impl Modificable<f32> for UnsafeCoordinate {
    fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    fn set_y(&mut self, y: f32) {
        self.y = y;
    }
}
