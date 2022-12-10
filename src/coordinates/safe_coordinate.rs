/*

Representation a pair of real-value, two dimensional SafeCoordinates, 
From now on, this structure will be referred as "SafeCoordinate".

    This class represents a two-dimensional SafeCoordinate. Its main purpose is to
    serve as a normalized conversion format for data going in and out of the
    scripts contained in the project.

    Parameters:
    -----------
    - x: float
        The x-SafeCoordinate.
    - y: float
        The y-SafeCoordinate.


        The use of this structure will be inmutable, so every operation over 
        a SafeCoordinate will bring a new SafeCoordinate, unless the unsafe version 
        of each operation, this will be done to let the user choose between different
        options and use the one seems the best for him.
*/

//mod gen_SafeCoordinate;
use super::gen_coordinate::Coordinate;


#[warn(non_camel_case_types)]
pub struct SafeCoordinate{
    x: f32,
    y: f32
}

impl Coordinate<f32> for SafeCoordinate{

    fn new (x: f32, y: f32) -> SafeCoordinate{
    return SafeCoordinate{x:x,y:y};
    }


    fn get_x(&self) -> f32 { self.x }

    fn get_y(&self) -> f32 { self.y }
    
    fn distancia(&self, altcoordinate: &SafeCoordinate) -> f32 {
        let dif_x:f32 = self.x - altcoordinate.x;
        let dif_y:f32 = self.y - altcoordinate.y;
        (dif_x.powi(2) + dif_y.powi(2)).sqrt()
    }
    
    fn negative(&self) -> SafeCoordinate{
        let new_c:SafeCoordinate= SafeCoordinate{x: -self.x , y: -self.y};
        return new_c;
    }
    
    fn add(&self, altcoordinate: &SafeCoordinate) -> SafeCoordinate{
        let new_c:SafeCoordinate= SafeCoordinate{x:self.x + altcoordinate.x, y:self.y + altcoordinate.y};
        return new_c;
    }
    
    fn sub(&self, altcoordinate: &SafeCoordinate) -> SafeCoordinate{
        let new_c:SafeCoordinate= SafeCoordinate{x:self.x - altcoordinate.x, y:self.y - altcoordinate.y};
        return new_c;
    }
    
    fn product(&self, altcoordinate: &SafeCoordinate) -> SafeCoordinate {
        let new_c:SafeCoordinate= SafeCoordinate{x:self.x * altcoordinate.x, y:self.y * altcoordinate.y};
        return new_c;
    }
    
    fn true_div(&self, altcoordinate: &SafeCoordinate) -> SafeCoordinate{
        let new_c:SafeCoordinate= SafeCoordinate{x:self.x * altcoordinate.x, y:self.y * (-altcoordinate.y)};
        return new_c;
    }

    fn equal(&self, altcoordinate: &SafeCoordinate) -> bool{
        return self.x == altcoordinate.x && self.y == altcoordinate.y;
    }

    fn c_mod(&self, altcoordinate: &SafeCoordinate) -> f32 {
        return self.x%altcoordinate.x + self.y%altcoordinate.y;
    }
    fn destroy(&self) ->() {
        drop(&self);
    }

}
//}