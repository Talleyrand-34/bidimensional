/*

Representation a pair of real-value, two dimensional coordinates, 
From now on, this structure will be referred as "Coordinate".

    This class represents a two-dimensional coordinate. Its main purpose is to
    serve as a normalized conversion format for data going in and out of the
    scripts contained in the project.

    Parameters:
    -----------
    - x: float
        The x-coordinate.
    - y: float
        The y-coordinate.


        The use of this structure will be inmutable, so every operation over 
        a Coordinate will bring a new Coordinate, unless the unsafe version 
        of each operation, this will be done to let the user choose between different
        options and use the one seems the best for him.
*/
//mod Coordinate{
pub struct Coordinate{
    x: f32,
    y: f32
}

impl Coordinate{

    fn new (x: f32, y: f32) -> Coordinate{
    return Coordinate{x:x,y:y};
    }


    fn get_x(&self) -> f32 { self.x }

    fn get_y(&self) -> f32 { self.y }
    
    fn distancia(&self, otro: &Coordinate) -> f32 {
        let dif_x = self.x - otro.x;
        let dif_y = self.y - otro.y;
        (dif_x.powi(2) + dif_y.powi(2)).sqrt()
    }
    
    fn negative(&self) -> Coordinate{
        let new_c:Coordinate= Coordinate{x: -self.x , y: -self.y};
        return new_c;
    }
    
    fn add(&self, otro: &Coordinate) -> Coordinate{
        let new_c:Coordinate= Coordinate{x:self.x + otro.x, y:self.y + otro.y};
        return new_c;
    }
    
    fn sub(&self, otro: &Coordinate) -> Coordinate{
        let new_c:Coordinate= Coordinate{x:self.x - otro.x, y:self.y - otro.y};
        return new_c;
    }
    
    fn prod(&self, otro: &Coordinate) -> Coordinate{
        let new_c:Coordinate= Coordinate{x:self.x * otro.x, y:self.y * otro.y};
        return new_c;
    }
    
    fn true_div(&self, otro: &Coordinate) -> Coordinate{
        let new_c:Coordinate= Coordinate{x:self.x * otro.x, y:self.y * (-otro.y)};
        return new_c;
    }

    fn equal(&self, otro: &Coordinate) -> bool{
        return self.x == otro.x && self.y == otro.y;
    }
}
//}