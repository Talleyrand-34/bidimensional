//use super::coordinate::coordinate;
use crate::ECoordinate ;
use std::ops::Mul;

pub struct PVec<T> where
T: Mul<Output = T> + Copy,{
    Vec:Vec<ECoordinate<T>>,
}