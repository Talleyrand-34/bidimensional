/*

Representation a pair of real-value, two dimensional UnsafeCoordinates,
From now on, this structure will be referred as "UnsafeCoordinate".

    This class represents a two-dimensional UnsafeCoordinate. Its main purpose is to
    serve as a normalized conversion format for data going in and out of the
    scripts contained in the project.

    Parameters:
    -----------
    - x: float
        The x-UnsafeCoordinate.
    - y: float
        The y-UnsafeCoordinate.


        The use of this structure will be inmutable, so every operation over
        a UnsafeCoordinate will bring a new SafeCoordinate, unless the unsafe version
        of each operation, this will be done to let the user choose between different
        options and use the one seems the best for him.
*/

//mod gen_UnsafeCoordinate;
// use super::gen_coordinate::CoordinateBasics;
// use super::gen_coordinate::UnmutableCoordinate;

use super::traits_coordinate::CoordinateBasics;
use super::traits_coordinate::MutableCoordinate;
// use self::traits_coordinate::CoordinateBasics;
// use self::traits_coordinate::UnmutableCoordinate;

#[derive(Clone, Copy, Debug)]
pub struct UnsafeCoordinate<T> {
    x: T,
    y: T,
}

impl<T> CoordinateBasics<T> for UnsafeCoordinate<T>
where
    T: Copy,
{
    fn new(x: T, y: T) -> UnsafeCoordinate<T> {
        UnsafeCoordinate { x, y }
    }

    fn get_x(&self) -> T {
        self.x
    }
    fn get_y(&self) -> T {
        self.y
    }
}
//
// impl<T: Copy + std::ops::Neg<Output=T> + std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::ops::Mul<Output=T> + std::ops::Div<Output=T>> UnmutableCoordinate<T> for UnsafeCoordinate<T> {
//     fn negative(&self) -> UnsafeCoordinate<T>{
//         UnsafeCoordinate { x: -self.x, y: -self.y }
//     }
//
//     fn add(&self, altcoordinate: &UnsafeCoordinate<T>) -> SafeCoordinate<T> {
//         UnsafeCoordinate { x: self.x + altcoordinate.x, y: self.y + altcoordinate.y }
//     }
//
//     fn sub(&self, altcoordinate: &UnsafeCoordinate<T>) -> SafeCoordinate<T> {
//         UnsafeCoordinate { x: self.x - altcoordinate.x, y: self.y - altcoordinate.y }
//     }
//
//     fn product(&self, altcoordinate: &UnsafeCoordinate<T>) -> SafeCoordinate<T> {
//         UnsafeCoordinate { x: self.x * altcoordinate.x, y: self.y * altcoordinate.y }
//     }
//
//     fn true_div(&self, altcoordinate: &UnsafeCoordinate<T>) -> SafeCoordinate<T> {
//         UnsafeCoordinate { x: self.x / altcoordinate.x, y: self.y / altcoordinate.y }
//     }
// }
impl<
        T: Copy
            + std::ops::Neg<Output = T>
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::AddAssign
            + std::ops::SubAssign
            + std::ops::MulAssign,
    > MutableCoordinate<T> for UnsafeCoordinate<T>
{
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
    fn set_x(&mut self, x: T) -> () {
        self.x = x;
    }
    fn set_y(&mut self, y: T) -> () {
        self.y = y;
    }
}

#[test]
fn test_unsafe_coordinate_basics() {
    //En los tests no se ponen los tipos
    let coord: UnsafeCoordinate<f32> = UnsafeCoordinate::new(1.0, 2.0);
    assert_eq!(1.0, coord.get_x());
    assert_eq!(2.0, coord.get_y());

}
#[test]
fn test_unsafe_coordinate_core() {
    let var_x: f32 = 1.0;
    let var_y: f32 = 2.0;
    let coordb: UnsafeCoordinate<f32> = UnsafeCoordinate::new(var_x, var_y);
    let mut coord: UnsafeCoordinate<f32> = UnsafeCoordinate::new(var_x, var_y);
    coord.negative();
    assert_eq!(-var_x, coord.get_x());
    assert_eq!(-var_y, coord.get_y());

    let mut coord: UnsafeCoordinate<f32> = UnsafeCoordinate::new(var_x, var_y);
    coord.add(&coordb);
    assert_eq!(2.0 * var_x, coord.get_x());
    assert_eq!(2.0 * var_y, coord.get_y());

    let mut coord: UnsafeCoordinate<f32> = UnsafeCoordinate::new(var_x, var_y);
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
    let coordb: UnsafeCoordinate<f32> = UnsafeCoordinate::new(var_x, var_y);
    let mut coord: UnsafeCoordinate<f32> = UnsafeCoordinate::new(var_x, var_y);
    coord.negative();
    assert_eq!(-var_x, coord.get_x());
    assert_eq!(-var_y, coord.get_y());

    let mut coord: UnsafeCoordinate<f32> = UnsafeCoordinate::new(var_x, var_y);
    coord.add(&coordb);
    assert_eq!(2.0 * var_x, coord.get_x());
    assert_eq!(2.0 * var_y, coord.get_y());

    let mut coord: UnsafeCoordinate<f32> = UnsafeCoordinate::new(var_x, var_y);
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
