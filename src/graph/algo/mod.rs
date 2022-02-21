pub mod dijkstra;
pub mod euler_tour;

// function for tests

use std::collections::{HashMap, HashSet};
pub type NodeId = usize;
pub fn undirected_neighbors(n: usize, edges: &[(usize, usize)]) -> HashMap<usize, HashSet<usize>> {
    let mut neighbors: HashMap<_, _> = (0..n).map(|i| (i, HashSet::new())).collect();
    for &(u, v) in edges {
        neighbors.get_mut(&u).expect("above added").insert(v);
        neighbors.get_mut(&v).expect("above added").insert(u);
    }
    neighbors
}
