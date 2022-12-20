// first thing to do is a getter to simplify code

use super::gen_coordinate::CoordinateBasics;
use super::unsafe_coordinate::UnsafeCoordinate;
use super::safe_coordinate::SafeCoordinate;

#[warn(dead_code)]
pub enum ECoordinate{
    Safe(SafeCoordinate),
    Unsafe(UnsafeCoordinate),
}


impl ECoordinate {
    
    

    /// checks if the coordinate is equal to the given one
    fn equal(&self, altcoordinate: &Self) -> bool{
        match (self, altcoordinate) {
            (ECoordinate::Safe(coord1), ECoordinate::Safe(coord2)) => coord1.get_x() == coord2.get_x() && coord1.get_y() == coord2.get_y(),
            (ECoordinate::Unsafe(coord1), ECoordinate::Unsafe(coord2)) => coord1.get_x() == coord2.get_x() && coord1.get_y() == coord2.get_y(),
            (ECoordinate::Safe(coord1), ECoordinate::Unsafe(coord2)) => coord1.get_x() == coord2.get_x() && coord1.get_y() == coord2.get_y(),
            (ECoordinate::Unsafe(coord1), ECoordinate::Safe(coord2)) => coord1.get_x() == coord2.get_x() && coord1.get_y() == coord2.get_y(),
            _ => false,
        }
        
    }
    /// checks if the direction of the vector is the same to the given one
    fn equiv(&self, altcoordinate: &Self) -> bool{
        let x:f32;
        let y:f32;

        if altcoordinate.x!=0.0 {
            x = self.x/altcoordinate.x;
        }
        else {
            x=0.0;
        }
        if altcoordinate.y!=0.0 {
            y= self.y/altcoordinate.y;
        }
        else {
            y=0.0;
         }

        return x==y;
    }
    /// returns the distance between the two coordinates
    fn distancia(&self, altcoordinate: &Self) -> f32{
        let dif_x:f32 = self.x - altcoordinate.x;
        let dif_y:f32 = self.y - altcoordinate.y;
        (dif_x.powi(2) + dif_y.powi(2)).sqrt()
    }
    // En el futuro esto se puede extender de la forma:
    //match self {
    //eCoordinate::Safe(safe_coord) => {
        // do something with the safe coordinate
    //}
    //eCoordinate::Unsafe(unsafe_coord) => {
        // do something with the unsafe coordinate
    //}
    //
    //
    

}
/*
match self {
            Coordinate::Safe(safe) => safe.new(x,y),
            Coordinate::Unsafe(unsafe) => unsafe.new(x,y),
        }
         */