You are correct that the std::num module does not have a NonZero type. However, the num-nonzero crate does provide a NonZero type that can be used with any unsigned or signed integer type.

To use the NonZero type from the num-nonzero crate, you will need to add it as a dependency in your Cargo.toml file:

[dependencies]
num-nonzero = "0.2.0"

Then, you can use the NonZero type like this:

use num_nonzero::NonZero;

pub struct Coordinate<T> {
    x: NonZero<T>,
    y: NonZero<T>,
}

impl<T> Coordinate<T> {
    pub fn new(x: T, y: T) -> Self
    where
        T: num::Num + Copy,
    {
        Self {
            x: NonZero::new(x).unwrap(),
            y: NonZero::new(y).unwrap(),
        }
    }
}

Note that the num-nonzero crate requires the num crate as a dependency, so you don't need to add it separately in your Cargo.toml file.

You can then create a Coordinate struct with either f32 or f64 fields like this:

let coordinate_f32 = Coordinate::new(1.0f32, 2.0f32);
let coordinate_f64 = Coordinate::new(1.0f64, 2.0f64);

Note that you need to specify the type parameter T when creating a Coordinate struct, because Rust cannot infer it from the arguments passed to the new function.


To implement operations that work with both f32 and f64 without duplicating code, you can use trait bounds. Trait bounds allow you to specify that a generic type parameter must implement certain traits.

For example, let's say you want to implement a distance function that calculates the distance between two points and works with both f32 and f64. You can do this using trait bounds as follows:

use std::ops::Sub;

struct Point<T> {
    x: T,
    y: T,
}

fn distance<T>(p1: Point<T>, p2: Point<T>) -> T
where
    T: Sub<Output = T> + Copy,
{
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    (dx * dx + dy * dy).sqrt()
}

fn main() {
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    let d = distance(p1, p2);
    println!("Distance between points: {}", d);
}

In this example, the distance function has a generic type parameter T, which represents the type of the x and y fields of the Point struct. The where clause specifies that T must implement the Sub trait (which provides the - operator) and the Copy trait. This allows the distance function to use the - operator to calculate the difference between the x and y values of the two points, and to copy the values of dx and dy as needed.

The distance function will work with both f32 and f64 because both types implement the Sub and Copy traits.

To ensure that a value has a certain type before it is assigned, you can use the std::convert::TryInto trait in Rust. This trait provides a method called try_into, which allows you to try to convert a value from one type to another. If the conversion is successful, it returns the converted value. If the conversion fails, it returns an error.

Here's an example of how you can use try_into to ensure that a value has a certain type before it is assigned:

use std::convert::TryInto;

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let x: Result<f64, _> = "3.0".try_into();
    let y: Result<f64, _> = "4.0".try_into();
    if let (Ok(x), Ok(y)) = (x, y) {
        let p = Point { x, y };
        println!("Point: ({}, {})", p.x, p.y);
    } else {
        panic!("Error: Unable to parse point coordinates!");
    }
}

In this example, the try_into method is used to try to convert the strings "3.0" and "4.0" to f64 values. If the conversion is successful, the x and y variables will contain the converted values, and the Point struct will be created with those values. If the conversion fails, the else block will be executed and the code will panic.

You can use try_into to ensure that a value has any type that implements the TryInto trait. For example, you can use it to ensure that a value is a valid integer, a valid floating-point number, or a valid string.

use std::marker::PhantomData;

struct MyStruct<T> {
    data: Vec<T>,
    marker: PhantomData<T>,
}

impl<T> MyStruct<T> {
    fn new() -> MyStruct<T> {
        MyStruct { data: Vec::new(), marker: PhantomData }
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}

fn main() {
    let mut my_struct = MyStruct::<i32>::new();
    my_struct.push(1);
    my_struct.push(2);
    my_struct.push(3);
    assert_eq!(my_struct.pop(), Some(3));
    assert_eq!(my_struct.pop(), Some(2));
    assert_eq!(my_struct.pop(), Some(1));
    assert_eq!(my_struct.pop(), None);
}
