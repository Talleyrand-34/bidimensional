// first thing to do is a getter to simplify code

use super::gen_coordinate::CoordinateBasics;
use super::unsafe_coordinate::UnsafeCoordinate;
use super::safe_coordinate::SafeCoordinate;

#[doc = "Esta es la enumeración `ECoordinate`."]

pub enum ECoordinate {
    /// Una variante que representa una coordenada segura.
    Safe(SafeCoordinate),
    /// Una variante que representa una coordenada insegura.
    Unsafe(UnsafeCoordinate),
}



impl ECoordinate {
    
    fn get_x(&self) -> Option<f32> {
        match self {
            ECoordinate::Safe(coord) => Some(coord.get_x()),
            ECoordinate::Unsafe(coord) => Some(coord.get_x()),
            _ => None,
        }
    }

    fn get_y(&self) -> Option<f32> {
        match self {
            ECoordinate::Safe(coord) => Some(coord.get_y()),
            ECoordinate::Unsafe(coord) => Some(coord.get_y()),
            _ => None,
        }
    }
    

    
    ///The funciton equiv:
///It compares two ECoordinates to see if they are equivalent. Two ECoordinates are 
/// equivalent if they have the same values for x and y. If either value is None, 
/// then they are considered not equivalent.
/// 
/// # Parameters
///
/// * `altcoordinate: A reference to another ECoordinate to compare to self.
///
/// # Returns
///
/// `true if self and altcoordinate are equivalent, false otherwise.
///
/// # Examples
///
/// /*```
/// let coord1 = ECoordinate::Safe(SafeCoordinate { x: 1.0, y: 2.0 });
/// let coord2 = ECoordinate::Safe(SafeCoordinate { x: 1.0, y: 2.0 });
/// assert!(coord1.equiv(&coord2));
///
/// let coord3 = ECoordinate::Safe(SafeCoordinate { x: 1.0, y: 2.0 });
/// let coord4 = ECoordinate::Safe(SafeCoordinate { x: 2.0, y: 2.0 });
/// assert!(!coord3.equiv(&coord4));
/// ```*/
/// 
    

fn equiv(&self, altcoordinate: &Self) -> bool {
        let x = match (self.get_x(), altcoordinate.get_x()) {
            (Some(self_x), Some(alt_x)) => self_x / alt_x,
            _ => 0.0,
        };
        let y = match (self.get_y(), altcoordinate.get_y()) {
            (Some(self_y), Some(alt_y)) => self_y / alt_y,
            _ => 0.0,
        };
        x == y
    }
    /// returns the distance between the two coordinates
    /// 
    fn distancia(&self, altcoordinate: &Self) -> f32 {
        match (self.get_x(), self.get_y(), altcoordinate.get_x(), altcoordinate.get_y()) {
            (Some(x1), Some(y1), Some(x2), Some(y2)) => {
                let dif_x = x1 - x2;
                let dif_y = y1 - y2;
                (dif_x.powi(2) + dif_y.powi(2)).sqrt()
            },
            _ => 0.0,
        }
    }

    fn equal(&self, coord2: &Self) -> bool {
        match (self.get_x(), self.get_y(), coord2.get_x(), coord2.get_y()) {
            (Some(x1), Some(y1), Some(x2), Some(y2)) => x1 == x2 && y1 == y2,
            _ => false,
        }
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



#[test]
fn test_safe_coordinate_operations() {
    let var_x1:f32=14.0;
    let var_y1:f32=20.0;
    let var_x2:f32=-13.0;
    let var_y2:f32=12.0;
    //En los tests no se ponen los tipos
    let coord1 : ECoordinate=  ECoordinate::Safe(SafeCoordinate::new(var_x1,var_y1));
    let coord2 :ECoordinate=ECoordinate::Safe(SafeCoordinate::new(var_x2,var_y2));
    let distancia: f32=coord1.distancia(&coord2);
    let ne:bool=coord1.equal(&coord2);
    let eq:bool=coord1.equal(&coord1);
    let c_mod:bool=coord1.equiv(&coord2);
    let c_mod_reg:bool=coord1.equiv(&coord1);

    assert_eq!(28.160255, distancia);
    assert_eq!(false,ne);
    assert_eq!(true,eq);
    assert_eq!(false,c_mod);
    assert_eq!(true,c_mod_reg);
}

#[test]
fn test_unsafe_coordinate_operations() {
    let var_x1:f32=14.0;
    let var_y1:f32=20.0;
    let var_x2:f32=-13.0;
    let var_y2:f32=12.0;
    //En los tests no se ponen los tipos
    let coord1 : ECoordinate=  ECoordinate::Unsafe(UnsafeCoordinate::new(var_x1,var_y1));
    let coord2 :ECoordinate=ECoordinate::Unsafe(UnsafeCoordinate::new(var_x2,var_y2));
    let distancia: f32=coord1.distancia(&coord2);
    let ne:bool=coord1.equal(&coord2);
    let eq:bool=coord1.equal(&coord1);
    let c_mod:bool=coord1.equiv(&coord2);
    let c_mod_reg:bool=coord1.equiv(&coord1);

    assert_eq!(28.160255, distancia);
    assert_eq!(false,ne);
    assert_eq!(true,eq);
    assert_eq!(false,c_mod);
    assert_eq!(true,c_mod_reg);
}