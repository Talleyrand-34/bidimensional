use super::vec_struct::PVec;
use super::adjancecy_list::AdjGraph;
use std::ops::Mul;
use crate::coordinates::coordinate::ECoordinate;
pub enum Polygon<T> where
    T: Mul<Output = T> + Copy,
    {
    PVec(PVec<T>),
    AdjGraph(AdjGraph<T>),
    //Array
    //LnkList
    //BHeap 
    }

impl<T> Polygon<T> where
T: Mul<Output = T> + Copy,{

    pub fn new() -> Self{
        Polygon::PVec(PVec::new())
    }

    pub fn new_vec() -> Self{
        Polygon::PVec(PVec::new())
    }

    pub fn new_adj_list()-> Self{
        Polygon::AdjGraph(AdjGraph::new())
    }

    pub fn add_point_single(&mut self, x:T, y:T) {
        //ECoordinate<T> new_point=ECoordinate::Safe(x,y);
        let new_point:ECoordinate<T>=ECoordinate::new_safe(x,y);
        match self{
            Polygon::PVec(p)=>{
                p.push(new_point);
            },
            Polygon::AdjGraph(adj)=>{
                adj.push(new_point);
            },
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
