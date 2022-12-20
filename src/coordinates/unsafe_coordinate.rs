//mod crate::coordinates::gen_coordinate;
//mod crate::gen_coordinate;
//use src::gen_coordinate;
//use crate::gen_coordinate::coordinate;


use super::gen_coordinate::CoordinateBasics;
use super::gen_coordinate::MutableCoordinate;
use super::gen_coordinate::OpCoordinates;

pub struct UnsafeCoordinate{
    x: f32,
    y: f32
}
impl CoordinateBasics<f32> for UnsafeCoordinate {
    fn new (x: f32, y: f32) -> UnsafeCoordinate{return UnsafeCoordinate{x:x,y:y};}
    fn get_x(&self) -> f32 { self.x }
    fn get_y(&self) -> f32 { self.y }
    fn destroy(&self) ->() {drop(self)}

}



impl MutableCoordinate<f32> for UnsafeCoordinate {
    
    fn negative(&mut self)-> (){
        self.x = -self.x;
        self.y = -self.y;
    }
    fn add(&mut self, altcoordinate: &Self) {
        self.x+=altcoordinate.x;
        self.y+=altcoordinate.y;
    }
    fn sub(&mut self, altcoordinate: &Self) -> () {
        self.x-=altcoordinate.x;
        self.y-=altcoordinate.y;
    }
    fn product(&mut self, altcoordinate: &Self) -> () {
        self.x*=altcoordinate.x;
        self.y*=altcoordinate.y;
    }
    
    fn true_div(&mut self, altcoordinate: &Self) -> () {
        self.x*=altcoordinate.x;
        self.y*=-altcoordinate.y;
    }
    fn set_x(&mut self, x: f32) -> () {
        self.x = x;
    }
    fn set_y(&mut self, y: f32) -> () {
        self.y= y;
    }
}


impl OpCoordinates<f32> for UnsafeCoordinate{
    fn distancia(&self, otro: &UnsafeCoordinate) -> f32 {
        let difx:f32 = self.x - otro.x;
        let dify:f32= self.y - otro.y;
        (difx.powi(2) + dify.powi(2)).sqrt()
    }
    fn equal(&self, altcoordinate: &Self) -> bool {
        return self.x == altcoordinate.x && self.y== altcoordinate.y;
    }
    fn equiv(&self, altcoordinate: &Self) -> bool {
        let x:f32;
        let y:f32;
        if altcoordinate.x!=0.0 {
            x = self.x/altcoordinate.x;
        }
        else if self.x!=0.0 {
            x = altcoordinate.x/self.x;
        }
        else {
            x=0.0;
        }
        if altcoordinate.y!=0.0 {
            y= self.y/altcoordinate.y;
        }
        else if self.y!=0.0 {
            y = altcoordinate.y/self.y;
        }
        else {
            y=0.0;
         }

        return x==y;
    }
}

#[test]
fn test_safe_coordinate_operations() {
    let var_x1:f32=14.0;
    let var_y1:f32=20.0;
    let var_x2:f32=-13.0;
    let var_y2:f32=12.0;
    //En los tests no se ponen los tipos
    let coord1 :UnsafeCoordinate=UnsafeCoordinate::new(var_x1,var_y1);
    let coord2 :UnsafeCoordinate=UnsafeCoordinate::new(var_x2,var_y2);
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
fn test_safe_coordinate_basics() {
    //En los tests no se ponen los tipos
    let coord :UnsafeCoordinate=UnsafeCoordinate::new(1.0,2.0);
    assert_eq!(1.0, coord.get_x());
    assert_eq!(2.0, coord.get_y());
    coord.destroy();
}

#[test]
fn test_safe_coordinate_core() {
    let var_x:f32=1.0;
    let var_y:f32=2.0;
    //En los tests no se ponen los tipos
    let mut coord :UnsafeCoordinate=UnsafeCoordinate::new(var_x,var_y);
    coord.negative();
    assert_eq!(-var_x,coord.get_x());
    assert_eq!(-var_y,coord.get_y());
    coord.set_x(var_x);
    coord.set_y(var_y);
    coord.add(&mut coord);
    assert_eq!(2.0*var_x,coord.get_x());
    assert_eq!(2.0*var_y,coord.get_y());
    coord.set_x(var_x);
    coord.set_y(var_y);
    coord.sub(&coord);
    assert_eq!(0.0,coord.get_x());
    assert_eq!(0.0,coord.get_y());
    coord.sub(&coord);
    assert_eq!(-var_x,coord.get_x());
    assert_eq!(-var_y,coord.get_y());
    coord.set_x(var_x);
    coord.set_y(var_y);
    coord.product(&coord);
    assert_eq!(var_x*var_x,coord.get_x());
    assert_eq!(var_y*var_y,coord.get_y());
    coord.set_x(var_x);
    coord.set_y(var_y);
    coord.true_div(&coord);
    assert_eq!(var_x*var_x,coord.get_x());
    assert_eq!(-var_y*var_y,coord.get_y());
    coord.destroy();
}

#[test]
fn test_safe_coordinate_core2() {
    let var_x:f32=54.0;
    let var_y:f32=-32.0;
    //En los tests no se ponen los tipos
    let coord :UnsafeCoordinate=UnsafeCoordinate::new(var_x,var_y);
    coord.negative();
    assert_eq!(-var_x,coord.get_x());
    assert_eq!(-var_y,coord.get_y());
    coord.set_x(var_x);
    coord.set_y(var_y);
    coord.add(&coord);
    assert_eq!(2.0*var_x,coord.get_x());
    assert_eq!(2.0*var_y,coord.get_y());
    coord.set_x(var_x);
    coord.set_y(var_y);
    coord.sub(&coord);
    assert_eq!(0.0,coord.get_x());
    assert_eq!(0.0,coord.get_y());
    coord.sub(&coord);
    assert_eq!(-var_x,coord.get_x());
    assert_eq!(-var_y,coord.get_y());
    coord.set_x(var_x);
    coord.set_y(var_y);
    coord.product(&coord);
    assert_eq!(var_x*var_x,coord.get_x());
    assert_eq!(var_y*var_y,coord.get_y());
    coord.set_x(var_x);
    coord.set_y(var_y);
    coord.true_div(&coord);
    assert_eq!(var_x*var_x,coord.get_x());
    assert_eq!(-var_y*var_y,coord.get_y());
    coord.destroy();
}