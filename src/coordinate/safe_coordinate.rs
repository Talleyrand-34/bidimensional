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
// use super::gen_coordinate::CoordinateBasics;
// use super::gen_coordinate::UnmutableCoordinate;

use super::traits_coordinate::CoordinateBasics;
use super::traits_coordinate::UnmutableCoordinate;
// use self::traits_coordinate::CoordinateBasics;
// use self::traits_coordinate::UnmutableCoordinate;

#[derive(Clone, Copy, Debug)]
pub struct SafeCoordinate<T> {
    x: T,
    y: T,
}

impl<T> CoordinateBasics<T> for SafeCoordinate<T>
where
    T: Copy,
{
    fn new(x: T, y: T) -> SafeCoordinate<T> {
        SafeCoordinate { x, y }
    }

    fn get_x(&self) -> T {
        self.x
    }
    fn get_y(&self) -> T {
        self.y
    }
}

impl<
        T: Copy
            + std::ops::Neg<Output = T>
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>,
    > UnmutableCoordinate<T> for SafeCoordinate<T>
{
    fn negative(&self) -> SafeCoordinate<T> {
        SafeCoordinate {
            x: -self.x,
            y: -self.y,
        }
    }

    fn add(&self, altcoordinate: &SafeCoordinate<T>) -> SafeCoordinate<T> {
        SafeCoordinate {
            x: self.x + altcoordinate.x,
            y: self.y + altcoordinate.y,
        }
    }

    fn sub(&self, altcoordinate: &SafeCoordinate<T>) -> SafeCoordinate<T> {
        SafeCoordinate {
            x: self.x - altcoordinate.x,
            y: self.y - altcoordinate.y,
        }
    }

    fn product(&self, altcoordinate: &SafeCoordinate<T>) -> SafeCoordinate<T> {
        SafeCoordinate {
            x: self.x * altcoordinate.x,
            y: self.y * altcoordinate.y,
        }
    }

    fn true_div(&self, altcoordinate: &SafeCoordinate<T>) -> SafeCoordinate<T> {
        SafeCoordinate {
            x: self.x / altcoordinate.x,
            y: self.y / altcoordinate.y,
        }
    }
}
#[test]
fn test_safe_coordinate_basics() {
    let coord = SafeCoordinate::new(1.0, 2.0);
    assert_eq!(1.0, coord.get_x());
    assert_eq!(2.0, coord.get_y());

}

#[test]
fn test_safe_coordinate_core() {
    let var_x = 1.0;
    let var_y = 2.0;
    let coord = SafeCoordinate::new(var_x, var_y);
    let negative = coord.negative();
    assert_eq!(-coord.get_x(), negative.get_x());
    assert_eq!(-coord.get_y(), negative.get_y());
    let addition = coord.add(&SafeCoordinate::new(var_x, var_y));
    assert_eq!(coord.get_x() + var_x, addition.get_x());
    assert_eq!(coord.get_y() + var_y, addition.get_y());
    let subtraction = coord.sub(&SafeCoordinate::new(var_x, var_y));
    assert_eq!(coord.get_x() - var_x, subtraction.get_x());
    assert_eq!(coord.get_y() - var_y, subtraction.get_y());
    let product = coord.product(&SafeCoordinate::new(var_x, var_y));
    assert_eq!(coord.get_x() * var_x, product.get_x());
    assert_eq!(coord.get_y() * var_y, product.get_y());
    let truediv = coord.true_div(&SafeCoordinate::new(var_x, var_y));
    assert_eq!(coord.get_x() / var_x, truediv.get_x());
    assert_eq!(coord.get_y() / var_y, truediv.get_y());

}

#[test]
fn test_safe_coordinate_core2() {
    let var_x = 54.0;
    let var_y = -2.0;
    let coord = SafeCoordinate::new(var_x, var_y);
    let negative = coord.negative();
    assert_eq!(-coord.get_x(), negative.get_x());
    assert_eq!(-coord.get_y(), negative.get_y());
    let addition = coord.add(&SafeCoordinate::new(var_x, var_y));
    assert_eq!(coord.get_x() + var_x, addition.get_x());
    assert_eq!(coord.get_y() + var_y, addition.get_y());
    let subtraction = coord.sub(&SafeCoordinate::new(var_x, var_y));
    assert_eq!(coord.get_x() - var_x, subtraction.get_x());
    assert_eq!(coord.get_y() - var_y, subtraction.get_y());
    let product = coord.product(&SafeCoordinate::new(var_x, var_y));
    assert_eq!(coord.get_x() * var_x, product.get_x());
    assert_eq!(coord.get_y() * var_y, product.get_y());
    let truediv = coord.true_div(&SafeCoordinate::new(var_x, var_y));
    assert_eq!(coord.get_x() / var_x, truediv.get_x());
    assert_eq!(coord.get_y() / var_y, truediv.get_y());

}
