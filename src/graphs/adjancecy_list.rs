
use std::ops::Mul;

struct Adj_Graph<T>
where
    T: Mul<Output = T> + Copy,
{
    adjacency_list: Vec<Vec<T>>,
}
