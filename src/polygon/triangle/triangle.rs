use crate::coordinates;
struct triangle<T>{
   x:T,
   y:T,
   z:T,
}

impl<T> triangle<T>{
    fn new(x:T, y:T, z:T) -> triangle<T>{
        triangle{
            x:x,
            y:y,
            z:z,
        }
    }

    fn new_from_array(coords: &[ECoordinate<T>]) -> triangle<T> {
    if coords.len() != 3 {
        panic!("Error: Not enough coordinates provided to create triangle.");
    }
    let x = coords[0];
    let y = coords[1];
    let z = coords[2];
    triangle { x, y, z }
    }


}