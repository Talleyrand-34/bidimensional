/*

Representation a pair of real-value, two dimensional Safef32coordinates, 
From now on, this structure will be referred as "Safef32coordinate".

    This class represents a two-dimensional Safef32coordinate. Its main purpose is to
    serve as a normalized conversion format for data going in and out of the
    scripts contained in the project.

    Parameters:
    -----------
    - x: float
        The x-Safef32coordinate.
    - y: float
        The y-Safef32coordinate.


        The use of this structure will be inmutable, so every operation over 
        a Safef32coordinate will bring a new Safef32coordinate, unless the unsafe version 
        of each operation, this will be done to let the user choose between different
        options and use the one seems the best for him.
*/





//mod gen_Safef32coordinate;
use super::traits_coordinate::CoordinateBasics;
use super::traits_coordinate::UnmutableCoordinate;



pub struct Safef32coordinate{
    x: f32,
    y: f32,
    //x: f32,
    //y: f32,
    //marker: PhantomData<f32>
}

impl CoordinateBasics<f32> for Safef32coordinate{

    fn new (x: f32, y: f32) -> Safef32coordinate{
    return Safef32coordinate{x:x,y:y};
    }
    fn get_x(&self) -> f32 { self.x }
    fn get_y(&self) -> f32 { self.y }
    fn destroy(&self) ->() {
        drop(&self);
    }

    
}




impl UnmutableCoordinate<f32> for Safef32coordinate{
    

    fn negative(&self) -> Safef32coordinate{
        let new_c:Safef32coordinate= Safef32coordinate{x: -self.x , y: -self.y};
        return new_c;
    }

    fn add(&self, altcoordinate: &Safef32coordinate) -> Safef32coordinate{
        let new_c:Safef32coordinate= Safef32coordinate{x:self.x + altcoordinate.x, y:self.y + altcoordinate.y};
        return new_c;
    }

    fn sub(&self, altcoordinate: &Safef32coordinate) -> Safef32coordinate{
        let new_c:Safef32coordinate= Safef32coordinate{x:self.x - altcoordinate.x, y:self.y - altcoordinate.y};
        return new_c;
    }

    fn product(&self, altcoordinate: &Safef32coordinate) -> Safef32coordinate {
        let new_c:Safef32coordinate= Safef32coordinate{x:self.x * altcoordinate.x, y:self.y * altcoordinate.y};
        return new_c;
    }

    fn true_div(&self, altcoordinate: &Safef32coordinate) -> Safef32coordinate{
        let new_c:Safef32coordinate= Safef32coordinate{x:self.x * altcoordinate.x, y:self.y * -altcoordinate.y};
        return new_c;
    }

    
}



#[test]
fn test_safe_coordinate_basics() {
    //En los tests no se ponen los tipos
    let coord :Safef32coordinate=Safef32coordinate::new(1.0,2.0);
    assert_eq!(1.0, coord.get_x());
    assert_eq!(2.0, coord.get_y());
    coord.destroy();
}

#[test]
fn test_safe_coordinate_core() {
    let var_x:f32=1.0;
    let var_y:f32=2.0;
    //En los tests no se ponen los tipos
    let coord :Safef32coordinate=Safef32coordinate::new(var_x,var_y);
    let negative: Safef32coordinate=coord.negative();
    assert_eq!(-coord.get_x(),negative.get_x());
    assert_eq!(-coord.get_y(),negative.get_y());
    let addition :Safef32coordinate= coord.add(&Safef32coordinate::new(var_x,var_y));
    assert_eq!(coord.get_x()+var_x,addition.get_x());
    assert_eq!(coord.get_y()+var_y,addition.get_y());
    let subtraction :Safef32coordinate= coord.sub(&Safef32coordinate::new(var_x,var_y));
    assert_eq!(coord.get_x()-var_x,subtraction.get_x());
    assert_eq!(coord.get_y()-var_y,subtraction.get_y());
    let product:Safef32coordinate=coord.product(&Safef32coordinate::new(var_x,var_y));
    assert_eq!(coord.get_x()*var_x,product.get_x());
    assert_eq!(coord.get_y()*var_y,product.get_y());
    let truediv:Safef32coordinate=coord.true_div(&Safef32coordinate::new(var_x,var_y));
    assert_eq!(coord.get_x()*var_x, truediv.get_x());
    assert_eq!(coord.get_y()*-var_y, truediv.get_y());
    coord.destroy();
}

#[test]
fn test_safe_coordinate_core2() {
    let var_x:f32=54.0;
    let var_y:f32=-32.0;
    //En los tests no se ponen los tipos
    let coord :Safef32coordinate=Safef32coordinate::new(var_x,var_y);
    let negative: Safef32coordinate=coord.negative();
    assert_eq!(-coord.get_x(),negative.get_x());
    assert_eq!(-coord.get_y(),negative.get_y());
    let addition :Safef32coordinate= coord.add(&Safef32coordinate::new(var_x,var_y));
    assert_eq!(coord.get_x()+var_x,addition.get_x());
    assert_eq!(coord.get_y()+var_y,addition.get_y());
    let subtraction :Safef32coordinate= coord.sub(&Safef32coordinate::new(var_x,var_y));
    assert_eq!(coord.get_x()-var_x,subtraction.get_x());
    assert_eq!(coord.get_y()-var_y,subtraction.get_y());
    let product:Safef32coordinate=coord.product(&Safef32coordinate::new(var_x,var_y));
    assert_eq!(coord.get_x()*var_x,product.get_x());
    assert_eq!(coord.get_y()*var_y,product.get_y());
    let truediv:Safef32coordinate=coord.true_div(&Safef32coordinate::new(var_x,var_y));
    assert_eq!(coord.get_x()*var_x, truediv.get_x());
    assert_eq!(coord.get_y()*-var_y, truediv.get_y());
    coord.destroy();
}