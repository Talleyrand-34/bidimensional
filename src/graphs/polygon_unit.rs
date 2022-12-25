use crate::PVec;

pub enum Polygon<T> {
    PVec(PVec<T>),
    //Array
    //LnkList
    //BHeap 
}

impl<T> Polygon<T> {
    pub fn new() -> Self {
        Polygon::PVec(PVec::new())
    }

    pub fn add_point_single(&mut self, x: T, y: T) {
        // add point to polygon
    }
}
use PVec;

pub enum Polygon<T>{
    PVec(PVec<T>),
    //Array
    //LnkList
    //BHeap 
}


impl Polygon{
    pub fn new() -> Self{
        Polygon::PVec(PVec::new());
    }

    pub fn new_vec() -> Self{
        Polygon::PVec(PVec::new());
    }

    pub fn add_point_single(&mut self, x:T, y:T) {
        match self{
            Polygon::PVec(p)=>{
                p.push(x);},
            _ => unimplemented!(),
        }
    }

    pub fn get_array(&self) -> &[ECoordinate<T>] {
        match self {
            Polygon::PVec(p) => p.Vec.as_slice(),
            _ => unimplemented!(),
        }
    }


    /*pub fn get_coordinates(&self){
        match self{
            Polygon::PVec(p)=>{

            }

    }*/


}
