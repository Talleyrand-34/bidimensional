
use super::f32_inmut_coordinate::Safef32coordinate;
use super::f32_mut_coordinate::Unsafef32coordinate;
use std::marker::PhantomData;
#[doc = "Esta es la enumeración `ECoordinate`."]
#[derive(Clone, Debug)]
pub enum ECoordinate<T>
//where T: Mul<Output = T> + Copy+ Add<Output = T>+Sub+ Div,
{
    Sf32(Safef32coordinate),
    Uf32(Unsafef32coordinate),
    // Una variante que representa una coordenada segura.
    // Safe(SafeCoordinate),
    // Una variante que representa una coordenada insegura.
    // Unsafe(UnsafeCoordinate),
    Phantom(PhantomData<T>),
    //
}

//These are the error codes to be called from functions
// static is_mut_err: &str=""
// static is_inmut_err: &str=""
impl ECoordinate<f32> {
    pub fn new_sf32(x: f32, y: f32) -> ECoordinate<f32> {
        ECoordinate::Sf32(Safef32coordinate::new(x, y))
    }

    pub fn new_uf32(x: f32, y: f32) -> ECoordinate<f32> {
        ECoordinate::Uf32(Unsafef32coordinate::new(x, y))
    }
}

impl<T> ECoordinate<T>
where
{
    pub fn new_safe(x: T, y: T) -> ECoordinate<T> {
        ECoordinate::Sf32(Safef32coordinate::new(x, y))

    }

    fn new_unsafe(x: T, y: T) -> ECoordinate<T> {
        ECoordinate::Uf32(Unsafef32coordinate::new(x, y))
        
//        ECoordinate::Unsafe(UnsafeCoordinate::new(x, y))
    }


    }
    fn get_x(&self) -> Result<T, &'static str> {
        match self {
            ECoordinate::Sf32(coordinate) => coordinate.get_x(),
            ECoordinate::Uf32(coordinate) => coordinate.get_x(),
            //ECoordinate::Safe(coord) => Some(coord.get_x()),
            //ECoordinate::Unsafe(coord) => Some(coord.get_x()),

            _ => Err("X coordinate not available for this ECoordinate variant"),
        }
    }


    fn get_y(&self) -> Result<T, &'static str> {
        match self {
            ECoordinate::Sf32(coordinate) => coordinate.get_y(),
            ECoordinate::Uf32(coordinate) => coordinate.get_y(),
            // ECoordinate::Safe(coord) => Some(coord.get_y()),
            // ECoordinate::Unsafe(coord) => Some(coord.get_y()),
            _ => Err("X coordinate not available for this ECoordinate variant"),
        }
    }
    

    fn negative(&self) -> Result<Self, &'static str> {
        match self {
            ECoordinate::Sf32(coordinate) => Ok(coordinate.negative()),
            ECoordinate::Uf32(coordinate) => Err("Permisions error: The operation should be mutable but is inmutable"),
//            ECoordinate::Safe(coord) => Some(coord.negative()),
//            ECoordinate::Safe(coord) => Some(ECoordinate::Safe(coord.negative())),
            _ => Err("X coordinate not available for this ECoordinate variant"),
        }
    }

    fn negative_mut(&mut self) -> bool {
        match self {
            ECoordinate::Uf32(coordinate) => {
                coordinate.negative();
                true
            },
            ECoordinate::Sf32(coordinate) => Err("Permisions error: The operation should be mutable but is inmutable"),
        }
    }

    fn add(&self, altcoordinate: &Self) -> Result<Self, &'static str> {
        match (self, altcoordinate) {
            ECoordinate::Sf32(coordinate) => Ok(coordinate.add(altcoordinate)),
            ECoordinate::Uf32(coordinate) => Err("Permisions error: The operation should be mutable but is inmutable"),
            // (ECoordinate::Safe(coord1), ECoordinate::Safe(coord2)) => {
            //     Some(ECoordinate::Safe(coord1.add(coord2)))
            // }
            _ => Err("X coordinate not available for this ECoordinate variant"),
        }
    }

    fn add_mut(&mut self, altcoordinate: &Self) -> bool {
        match (self, altcoordinate) {
            // (ECoordinate::Unsafe(ref mut coord1), ECoordinate::Unsafe(coord2)) => {
            //     coord1.add(coord2);
            //     true
            // }
            _ => false,
        }
    }
    fn sub(&self, altcoordinate: &Self) -> Result<Self, &'static str> {
        match (self, altcoordinate) {
            ECoordinate::Sf32(coordinate) => Ok(coordinate.sub(altcoordinate)),
            ECoordinate::Uf32(coordinate) => Err("Permisions error: The operation should be mutable but is inmutable"),
            // (ECoordinate::Safe(coord1), ECoordinate::Safe(coord2)) => {
            //     Some(ECoordinate::Safe(coord1.sub(coord2)))
            // }
            _ => Err("X coordinate not available for this ECoordinate variant"),
        }
    }

    fn sub_mut(&mut self, altcoordinate: &Self) -> bool {
        match (self, altcoordinate) {
            // (ECoordinate::Unsafe(ref mut coord1), ECoordinate::Unsafe(coord2)) => {
            //     coord1.sub(coord2);
            //     true
            // }
            _ => false,
        }
    }
    fn product(&self, altcoordinate: &Self) -> Result<Self, &'static str> {
        match (self, altcoordinate) {
            ECoordinate::Sf32(coordinate) => Ok(coordinate.product(altcoordinate)),
            ECoordinate::Uf32(coordinate) => Err("Permisions error: The operation should be mutable but is inmutable"),
            // (ECoordinate::Safe(coord1), ECoordinate::Safe(coord2)) => {
            //     Some(ECoordinate::Safe(coord1.product(coord2)))
            // }
            _ => Err("X coordinate not available for this ECoordinate variant"),
        }
    }

    fn product_mut(&mut self, altcoordinate: &Self) -> bool {
        match (self, altcoordinate) {
            // (ECoordinate::Unsafe(ref mut coord1), ECoordinate::Unsafe(coord2)) => {
            //     coord1.product(coord2);
            //     true
            // }
            _ => false,
        }
    }

    fn true_div(&self, altcoordinate: &Self) -> Result<Self, &'static str> {
        match (self, altcoordinate) {
            ECoordinate::Sf32(coordinate) => Ok(coordinate.true_div(altcoordinate)),
            ECoordinate::Uf32(coordinate) => Err("Permisions error: The operation should be mutable but is inmutable"),
            // (ECoordinate::Safe(coord1), ECoordinate::Safe(coord2)) => {
            //     Some(ECoordinate::Safe(coord1.true_div(coord2)))
            // }
            _ => Err("X coordinate not available for this ECoordinate variant"),
        }
    }

    fn true_div_mut(&mut self, altcoordinate: &Self) -> bool {
        match (self, altcoordinate) {
            // (ECoordinate::Unsafe(ref mut coord1), ECoordinate::Unsafe(coord2)) => {
            //     coord1.true_div(coord2);
            //     true
            // }
            _ => false,
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
        match (
            self.get_x(),
            self.get_y(),
            altcoordinate.get_x(),
            altcoordinate.get_y(),
        ) {
            (Some(x1), Some(y1), Some(x2), Some(y2)) => {
                let dif_x = x1 - x2;
                let dif_y = y1 - y2;
                (dif_x.powi(2) + dif_y.powi(2)).sqrt()
            }
            _ => 0.0,
        }
    }

    fn equal(&self, coord2: &Self) -> bool {
        match (self.get_x(), self.get_y(), coord2.get_x(), coord2.get_y()) {
            (Some(x1), Some(y1), Some(x2), Some(y2)) => x1 == x2 && y1 == y2,
            _ => false,
        }
    }

    fn midpoint(&self, coord2: &Self) -> Result<ECoordinate<T>,& 'static str> {
        match (self.get_x(), self.get_y(), coord2.get_x(), coord2.get_y()) {
            (Some(x1), Some(y1), Some(x2), Some(y2)) => Some(ECoordinate::Safe(SafeCoordinate::new((x1 + x2) / 2.0, (y1 + y2) / 2.0))),
            _ => None,
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

#[test]
pub fn test_f32_safe_coordinate_operations() {
    let var_x1: f32 = 14.0;
    let var_y1: f32 = 20.0;
    let var_x2: f32 = -13.0;
    let var_y2: f32 = 12.0;
    //En los tests no se ponen los tipos
    let coord1: ECoordinate = Sf32(Safef32coordinate.new(var_x1,var_y1));
    let coord2: ECoordinate = Sf32(Safef32coordinate.new(var_x2,var_y2));

    let coord3: ECoordinate = coord1.add(&coord2).unwrap();
    let coord4: ECoordinate = coord1.sub(&coord2).unwrap();
    let coord5: ECoordinate = coord1.product(&coord2).unwrap();
    let coord6: ECoordinate = coord1.true_div(&coord2).unwrap();

    assert_eq!(var_x1 + var_x2, coord3.get_x().unwrap());
    assert_eq!(var_y1 + var_y2, coord3.get_y().unwrap());
    assert_eq!(var_x1 - var_x2, coord4.get_x().unwrap());
    assert_eq!(var_y1 - var_y2, coord4.get_y().unwrap());
    assert_eq!(var_x1 * var_x2, coord5.get_x().unwrap());
    assert_eq!(var_y1 * var_y2, coord5.get_y().unwrap());
    assert_eq!(var_x1 * var_x2, coord6.get_x().unwrap());
    assert_eq!(var_y1 * (-var_y2), coord6.get_y().unwrap());
}

#[test]
pub fn test_unsafe_coordinate_basic_operations() {
    let var_x1: f32 = 14.0;
    let var_y1: f32 = 20.0;
    let var_x2: f32 = -13.0;
    let var_y2: f32 = 12.0;
    //En los tests no se ponen los tipos
    //let mut coord1 : ECoordinate=  ECoordinate::Unsafe(UnsafeCoordinate::new(var_x1,var_y1));
    let coord2: ECoordinate = ECoordinate::Unsafe(UnsafeCoordinate::new(var_x2, var_y2));

    let mut coord3: ECoordinate = ECoordinate::Unsafe(UnsafeCoordinate::new(var_x1, var_y1)); //coord1.add(&coord2).unwrap();
    let mut coord4: ECoordinate = ECoordinate::Unsafe(UnsafeCoordinate::new(var_x1, var_y1)); //coord1.sub(&coord2).unwrap();
    let mut coord5: ECoordinate = ECoordinate::Unsafe(UnsafeCoordinate::new(var_x1, var_y1)); //coord1.product(&coord2).unwrap();
    let mut coord6: ECoordinate = ECoordinate::Unsafe(UnsafeCoordinate::new(var_x1, var_y1)); //coord1.true_div(&coord2).unwrap();
    coord3.add_mut(&coord2);
    coord4.sub_mut(&coord2);
    coord5.product_mut(&coord2);
    coord6.true_div_mut(&coord2);
    assert_eq!(var_x1 + var_x2, coord3.get_x().unwrap());
    assert_eq!(var_y1 + var_y2, coord3.get_y().unwrap());
    assert_eq!(var_x1 - var_x2, coord4.get_x().unwrap());
    assert_eq!(var_y1 - var_y2, coord4.get_y().unwrap());
    assert_eq!(var_x1 * var_x2, coord5.get_x().unwrap());
    assert_eq!(var_y1 * var_y2, coord5.get_y().unwrap());
    assert_eq!(var_x1 * var_x2, coord6.get_x().unwrap());
    assert_eq!(var_y1 * (-var_y2), coord6.get_y().unwrap());
}

#[test]
pub fn test_safe_2_coordinate_operations() {
    let var_x1: f32 = 14.0;
    let var_y1: f32 = 20.0;
    let var_x2: f32 = -13.0;
    let var_y2: f32 = 12.0;
    //En los tests no se ponen los tipos
    let coord1: ECoordinate = ECoordinate::Safe(SafeCoordinate::new(var_x1, var_y1));
    let coord2: ECoordinate = ECoordinate::Safe(SafeCoordinate::new(var_x2, var_y2));
    let distancia: f32 = coord1.distancia(&coord2);
    let ne: bool = coord1.equal(&coord2);
    let eq: bool = coord1.equal(&coord1);
    let c_mod: bool = coord1.equiv(&coord2);
    let c_mod_reg: bool = coord1.equiv(&coord1);

    assert_eq!(28.160255, distancia);
    assert_eq!(false, ne);
    assert_eq!(true, eq);
    assert_eq!(false, c_mod);
    assert_eq!(true, c_mod_reg);
}

#[test]
pub fn test_unsafe_2_coord_operations() {
    let var_x1: f32 = 14.0;
    let var_y1: f32 = 20.0;
    let var_x2: f32 = -13.0;
    let var_y2: f32 = 12.0;
    //En los tests no se ponen los tipos
    let coord1: ECoordinate = ECoordinate::Unsafe(UnsafeCoordinate::new(var_x1, var_y1));
    let coord2: ECoordinate = ECoordinate::Unsafe(UnsafeCoordinate::new(var_x2, var_y2));
    let distancia: f32 = coord1.distancia(&coord2);
    let ne: bool = coord1.equal(&coord2);
    let eq: bool = coord1.equal(&coord1);
    let c_mod: bool = coord1.equiv(&coord2);
    let c_mod_reg: bool = coord1.equiv(&coord1);

    assert_eq!(28.160255, distancia);
    assert_eq!(false, ne);
    assert_eq!(true, eq);
    assert_eq!(false, c_mod);
    assert_eq!(true, c_mod_reg);
}
//
// #[test]
// pub fn test_safe_coordinate_operations() {
//     let var_x1: f32 = 14.0;
//     let var_y1: f32 = 20.0;
//     let var_x2: f32 = -13.0;
//     let var_y2: f32 = 12.0;
//     //En los tests no se ponen los tipos
//     let coord1: ECoordinate = ECoordinate::Safe(SafeCoordinate::new(var_x1, var_y1));
//     let coord2: ECoordinate = ECoordinate::Safe(SafeCoordinate::new(var_x2, var_y2));
//
//     let coord3: ECoordinate = coord1.add(&coord2).unwrap();
//     let coord4: ECoordinate = coord1.sub(&coord2).unwrap();
//     let coord5: ECoordinate = coord1.product(&coord2).unwrap();
//     let coord6: ECoordinate = coord1.true_div(&coord2).unwrap();
//
//     assert_eq!(var_x1 + var_x2, coord3.get_x().unwrap());
//     assert_eq!(var_y1 + var_y2, coord3.get_y().unwrap());
//     assert_eq!(var_x1 - var_x2, coord4.get_x().unwrap());
//     assert_eq!(var_y1 - var_y2, coord4.get_y().unwrap());
//     assert_eq!(var_x1 * var_x2, coord5.get_x().unwrap());
//     assert_eq!(var_y1 * var_y2, coord5.get_y().unwrap());
//     assert_eq!(var_x1 * var_x2, coord6.get_x().unwrap());
//     assert_eq!(var_y1 * (-var_y2), coord6.get_y().unwrap());
// }
//
// #[test]
// pub fn test_unsafe_coordinate_basic_operations() {
//     let var_x1: f32 = 14.0;
//     let var_y1: f32 = 20.0;
//     let var_x2: f32 = -13.0;
//     let var_y2: f32 = 12.0;
//     //En los tests no se ponen los tipos
//     //let mut coord1 : ECoordinate=  ECoordinate::Unsafe(UnsafeCoordinate::new(var_x1,var_y1));
//     let coord2: ECoordinate = ECoordinate::Unsafe(UnsafeCoordinate::new(var_x2, var_y2));
//
//     let mut coord3: ECoordinate = ECoordinate::Unsafe(UnsafeCoordinate::new(var_x1, var_y1)); //coord1.add(&coord2).unwrap();
//     let mut coord4: ECoordinate = ECoordinate::Unsafe(UnsafeCoordinate::new(var_x1, var_y1)); //coord1.sub(&coord2).unwrap();
//     let mut coord5: ECoordinate = ECoordinate::Unsafe(UnsafeCoordinate::new(var_x1, var_y1)); //coord1.product(&coord2).unwrap();
//     let mut coord6: ECoordinate = ECoordinate::Unsafe(UnsafeCoordinate::new(var_x1, var_y1)); //coord1.true_div(&coord2).unwrap();
//     coord3.add_mut(&coord2);
//     coord4.sub_mut(&coord2);
//     coord5.product_mut(&coord2);
//     coord6.true_div_mut(&coord2);
//     assert_eq!(var_x1 + var_x2, coord3.get_x().unwrap());
//     assert_eq!(var_y1 + var_y2, coord3.get_y().unwrap());
//     assert_eq!(var_x1 - var_x2, coord4.get_x().unwrap());
//     assert_eq!(var_y1 - var_y2, coord4.get_y().unwrap());
//     assert_eq!(var_x1 * var_x2, coord5.get_x().unwrap());
//     assert_eq!(var_y1 * var_y2, coord5.get_y().unwrap());
//     assert_eq!(var_x1 * var_x2, coord6.get_x().unwrap());
//     assert_eq!(var_y1 * (-var_y2), coord6.get_y().unwrap());
// }
//
// #[test]
// pub fn test_safe_2_coordinate_operations() {
//     let var_x1: f32 = 14.0;
//     let var_y1: f32 = 20.0;
//     let var_x2: f32 = -13.0;
//     let var_y2: f32 = 12.0;
//     //En los tests no se ponen los tipos
//     let coord1: ECoordinate = ECoordinate::Safe(SafeCoordinate::new(var_x1, var_y1));
//     let coord2: ECoordinate = ECoordinate::Safe(SafeCoordinate::new(var_x2, var_y2));
//     let distancia: f32 = coord1.distancia(&coord2);
//     let ne: bool = coord1.equal(&coord2);
//     let eq: bool = coord1.equal(&coord1);
//     let c_mod: bool = coord1.equiv(&coord2);
//     let c_mod_reg: bool = coord1.equiv(&coord1);
//
//     assert_eq!(28.160255, distancia);
//     assert_eq!(false, ne);
//     assert_eq!(true, eq);
//     assert_eq!(false, c_mod);
//     assert_eq!(true, c_mod_reg);
// }
//
// #[test]
// pub fn test_unsafe_2_coord_operations() {
//     let var_x1: f32 = 14.0;
//     let var_y1: f32 = 20.0;
//     let var_x2: f32 = -13.0;
//     let var_y2: f32 = 12.0;
//     //En los tests no se ponen los tipos
//     let coord1: ECoordinate = ECoordinate::Unsafe(UnsafeCoordinate::new(var_x1, var_y1));
//     let coord2: ECoordinate = ECoordinate::Unsafe(UnsafeCoordinate::new(var_x2, var_y2));
//     let distancia: f32 = coord1.distancia(&coord2);
//     let ne: bool = coord1.equal(&coord2);
//     let eq: bool = coord1.equal(&coord1);
//     let c_mod: bool = coord1.equiv(&coord2);
//     let c_mod_reg: bool = coord1.equiv(&coord1);
//
//     assert_eq!(28.160255, distancia);
//     assert_eq!(false, ne);
//     assert_eq!(true, eq);
//     assert_eq!(false, c_mod);
//     assert_eq!(true, c_mod_reg);
// }
