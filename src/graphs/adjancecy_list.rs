
use std::ops::Mul;

pub struct AdjGraph<T>
where
    T: Mul<Output = T> + Copy,
{
    adjacency_list: Vec<Vec<T>>,
}
