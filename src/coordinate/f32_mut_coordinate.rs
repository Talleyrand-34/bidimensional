//mod crate::coordinates::traits_coordinate;
//mod crate::traits_coordinate;
//use src::traits_coordinate;
//use crate::traits_coordinate::coordinate;

use super::traits_coordinate::CoordinateBasics;
use super::traits_coordinate::MutableCoordinate;

#[derive(Clone, Debug)]
pub struct Unsafef32coordinate {
    x: f32,
    y: f32,
}
impl CoordinateBasics<f32> for Unsafef32coordinate {
    fn new(x: f32, y: f32) -> Unsafef32coordinate {
        return Unsafef32coordinate { x: x, y: y };
    }
    fn get_x(&self) -> f32 {
        self.x
    }
    fn get_y(&self) -> f32 {
        self.y
    }
}

impl MutableCoordinate<f32> for Unsafef32coordinate {
    fn negative(&mut self) -> () {
        self.x = -self.x;
        self.y = -self.y;
    }
    fn add(&mut self, altcoordinate: &Self) {
        self.x += altcoordinate.x;
        self.y += altcoordinate.y;
    }
    fn sub(&mut self, altcoordinate: &Self) -> () {
        self.x -= altcoordinate.x;
        self.y -= altcoordinate.y;
    }
    fn product(&mut self, altcoordinate: &Self) -> () {
        self.x *= altcoordinate.x;
        self.y *= altcoordinate.y;
    }

    fn true_div(&mut self, altcoordinate: &Self) -> () {
        self.x *= altcoordinate.x;
        self.y *= -altcoordinate.y;
    }
    fn set_x(&mut self, x: f32) -> () {
        self.x = x;
    }
    fn set_y(&mut self, y: f32) -> () {
        self.y = y;
    }
}

#[test]
fn test_unsafe_coordinate_basics() {
    //En los tests no se ponen los tipos
    let coord: Unsafef32coordinate = Unsafef32coordinate::new(1.0, 2.0);
    assert_eq!(1.0, coord.get_x());
    assert_eq!(2.0, coord.get_y());
    
}
#[test]
fn test_unsafe_coordinate_core() {
    let var_x: f32 = 1.0;
    let var_y: f32 = 2.0;
    let coordb: Unsafef32coordinate = Unsafef32coordinate::new(var_x, var_y);
    let mut coord: Unsafef32coordinate = Unsafef32coordinate::new(var_x, var_y);
    coord.negative();
    assert_eq!(-var_x, coord.get_x());
    assert_eq!(-var_y, coord.get_y());

    let mut coord: Unsafef32coordinate = Unsafef32coordinate::new(var_x, var_y);
    coord.add(&coordb);
    assert_eq!(2.0 * var_x, coord.get_x());
    assert_eq!(2.0 * var_y, coord.get_y());

    let mut coord: Unsafef32coordinate = Unsafef32coordinate::new(var_x, var_y);
    coord.sub(&coordb);
    assert_eq!(0.0, coord.get_x());
    assert_eq!(0.0, coord.get_y());

    coord.sub(&coordb);
    assert_eq!(-var_x, coord.get_x());
    assert_eq!(-var_y, coord.get_y());

    coord.set_x(var_x);
    coord.set_y(var_y);
    coord.product(&coordb);
    assert_eq!(var_x * var_x, coord.get_x());
    assert_eq!(var_y * var_y, coord.get_y());

    coord.set_x(var_x);
    coord.set_y(var_y);
    coord.true_div(&coordb);
    assert_eq!(var_x * var_x, coord.get_x());
    assert_eq!(var_y * (-var_y), coord.get_y());

    
    
}

#[test]
fn test_unsafe_coordinate_core2() {
    let var_x: f32 = 54.0;
    let var_y: f32 = -32.0;
    let coordb: Unsafef32coordinate = Unsafef32coordinate::new(var_x, var_y);
    let mut coord: Unsafef32coordinate = Unsafef32coordinate::new(var_x, var_y);
    coord.negative();
    assert_eq!(-var_x, coord.get_x());
    assert_eq!(-var_y, coord.get_y());

    let mut coord: Unsafef32coordinate = Unsafef32coordinate::new(var_x, var_y);
    coord.add(&coordb);
    assert_eq!(2.0 * var_x, coord.get_x());
    assert_eq!(2.0 * var_y, coord.get_y());

    let mut coord: Unsafef32coordinate = Unsafef32coordinate::new(var_x, var_y);
    coord.sub(&coordb);
    assert_eq!(0.0, coord.get_x());
    assert_eq!(0.0, coord.get_y());

    coord.sub(&coordb);
    assert_eq!(-var_x, coord.get_x());
    assert_eq!(-var_y, coord.get_y());

    coord.set_x(var_x);
    coord.set_y(var_y);
    coord.product(&coordb);
    assert_eq!(var_x * var_x, coord.get_x());
    assert_eq!(var_y * var_y, coord.get_y());

    coord.set_x(var_x);
    coord.set_y(var_y);
    coord.true_div(&coordb);
    assert_eq!(var_x * var_x, coord.get_x());
    assert_eq!(var_y * (-var_y), coord.get_y());

    
    
}
