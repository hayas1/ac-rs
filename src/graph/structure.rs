use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::ops::Index;

pub enum Undirected {}

pub enum Directed {}

// graph structure
pub struct AdjacencyList<W, D> {
    neighbors: HashMap<usize, HashSet<usize>>,
    weight: HashMap<(usize, usize), W>,
    directed: PhantomData<D>,
}

// unweighted undirected graph structure
impl AdjacencyList<(), Undirected> {
    /// **O(m)**, convert sequence of edges(unweighted, undirected) to adjacency list
    pub fn new_unweighted_undirected((n, _m): (usize, usize), e: &[(usize, usize)]) -> Self {
        let e: Vec<_> = e.iter().map(|&(u, v)| (u, v, ())).collect();
        Self::new_weighted_undirected((n, _m), &e)
    }
}

// unweighted directed graph structure
impl AdjacencyList<(), Directed> {
    /// **O(m)**, convert sequence of edges(unweighted, undirected) to adjacency list
    pub fn new_unweighted_directed((n, _m): (usize, usize), e: &[(usize, usize)]) -> Self {
        let e: Vec<_> = e.iter().map(|&(u, v)| (u, v, ())).collect();
        Self::new_weighted_directed((n, _m), &e)
    }
}

// weighted undirected graph structure
impl<W: Copy> AdjacencyList<W, Undirected> {
    /// **O(m)**, convert sequence of edges(weighted, undirected) to adjacency list
    pub fn new_weighted_undirected((n, _m): (usize, usize), e: &[(usize, usize, W)]) -> Self {
        let mut weight = HashMap::new();
        let mut neighbors: HashMap<_, _> = (0..n).map(|u| (u, HashSet::new())).collect();
        for &(u, v, w) in e {
            neighbors
                .get_mut(&u)
                .unwrap_or_else(|| panic!("unexpected node: {}", u)) // expect will cost format every time
                .insert(v);
            neighbors
                .get_mut(&v)
                .unwrap_or_else(|| panic!("unexpected node: {}", u)) // expect will cost format every time
                .insert(u);
            weight.insert((u, v), w);
            weight.insert((v, u), w);
        }
        Self {
            neighbors,
            weight,
            directed: PhantomData,
        }
    }
}

// weighted directed graph structure
impl<W: Copy> AdjacencyList<W, Directed> {
    /// **O(m)**, convert sequence of edges(weighted, undirected) to adjacency list
    pub fn new_weighted_directed((n, _m): (usize, usize), e: &[(usize, usize, W)]) -> Self {
        let mut weight = HashMap::new();
        let mut neighbors: HashMap<_, _> = (0..n).map(|u| (u, HashSet::new())).collect();
        for &(u, v, w) in e {
            neighbors
                .get_mut(&u)
                .unwrap_or_else(|| panic!("unexpected node: {}", u)) // expect will cost format every time
                .insert(v);
            weight.insert((u, v), w);
        }
        Self {
            neighbors,
            weight,
            directed: PhantomData,
        }
    }
}

// weighted graph structure
impl<W: Copy, D> AdjacencyList<W, D> {
    /// **O(1)**, get weight of edge
    pub fn weight(&self, u: usize, v: usize) -> W {
        self.weight[&(u, v)]
    }
}
impl<W: Copy, D> Index<(usize, usize)> for AdjacencyList<W, D> {
    type Output = W;

    #[inline]
    fn index(&self, (u, v): (usize, usize)) -> &Self::Output {
        &self.weight[&(u, v)]
    }
}

// graph structure
impl<W, D> AdjacencyList<W, D> {
    /// **O(1)**, get node's neighbors list reference
    pub fn neighbors(&self, node: usize) -> &HashSet<usize> {
        &self.neighbors[&node]
    }

    /// **O(1)**, number of nodes
    pub fn nodes_len(&self) -> usize {
        self.neighbors.len()
    }
}
impl<W: Copy, D> Index<usize> for AdjacencyList<W, D> {
    type Output = HashSet<usize>;

    #[inline]
    fn index(&self, node: usize) -> &Self::Output {
        &self.neighbors[&node]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_unweighted_undirected() {
        // 1-2
        // \ /
        //  0
        // / \
        // 3-4
        let (n, m) = (5, 6);
        let e = vec![(0, 1), (1, 2), (2, 0), (0, 3), (3, 4), (4, 0)];
        let adjacency_list = AdjacencyList::new_unweighted_undirected((n, m), &e);
        assert_eq!(adjacency_list.nodes_len(), 5);
        assert_eq!(
            adjacency_list.neighbors(0),
            &[1, 2, 3, 4].iter().cloned().collect()
        );
        assert_eq!(
            adjacency_list.neighbors(1),
            &[0, 2].iter().cloned().collect()
        );
        assert_eq!(
            adjacency_list.neighbors(2),
            &[0, 1].iter().cloned().collect()
        );
        assert_eq!(adjacency_list[3], [0, 4].iter().cloned().collect());
        assert_eq!(adjacency_list[4], [0, 3].iter().cloned().collect());
        assert_eq!(adjacency_list[(1, 2)], ());
        assert_eq!(adjacency_list.weight(1, 2), ());
    }

    #[test]
    fn test_unweighted_directed() {
        // 1-2
        // \ /
        //  0
        // / \
        // 3-4
        let (n, m) = (5, 6);
        let e = vec![(0, 1), (1, 2), (2, 0), (0, 3), (3, 4), (4, 0)];
        let adjacency_list = AdjacencyList::new_unweighted_directed((n, m), &e);
        assert_eq!(adjacency_list.nodes_len(), 5);
        assert_eq!(
            adjacency_list.neighbors(0),
            &[1, 3].iter().cloned().collect()
        );
        assert_eq!(adjacency_list.neighbors(1), &[2].iter().cloned().collect());
        assert_eq!(adjacency_list.neighbors(2), &[0].iter().cloned().collect());
        assert_eq!(adjacency_list[3], [4].iter().cloned().collect());
        assert_eq!(adjacency_list[4], [0].iter().cloned().collect());
        assert_eq!(adjacency_list[(3, 4)], ());
        assert_eq!(adjacency_list.weight(3, 4), ());
    }

    #[test]
    fn test_weighted_undirected() {
        //     3
        //    1-2
        //  1 \ / 2
        //     0
        //  3 / \ 4
        //    3-4
        //     7
        let (n, m) = (5, 6);
        let e = vec![
            (0, 1, 1),
            (1, 2, 3),
            (2, 0, 2),
            (0, 3, 3),
            (3, 4, 7),
            (4, 0, 4),
        ];
        let adjacency_list = AdjacencyList::new_weighted_undirected((n, m), &e);
        assert_eq!(adjacency_list.nodes_len(), 5);
        assert_eq!(
            adjacency_list.neighbors(0),
            &[1, 2, 3, 4].iter().cloned().collect()
        );
        assert_eq!(
            adjacency_list.neighbors(1),
            &[0, 2].iter().cloned().collect()
        );
        assert_eq!(
            adjacency_list.neighbors(2),
            &[0, 1].iter().cloned().collect()
        );
        assert_eq!(adjacency_list[3], [0, 4].iter().cloned().collect());
        assert_eq!(adjacency_list[4], [0, 3].iter().cloned().collect());
        assert_eq!(adjacency_list.weight(0, 1), 1);
        assert_eq!(adjacency_list.weight(1, 0), 1);
        assert_eq!(adjacency_list.weight(3, 4), 7);
        assert_eq!(adjacency_list[(0, 1)], 1);
        assert_eq!(adjacency_list[(1, 0)], 1);
        assert_eq!(adjacency_list[(3, 4)], 7);
    }

    #[test]
    fn test_weighted_directed() {
        //     3
        //    1-2
        //  1 \ / 2
        //     0
        //  3 / \ 4
        //    3-4
        //     7
        let (n, m) = (5, 6);
        let e = vec![
            (0, 1, 1),
            (1, 2, 3),
            (2, 0, 2),
            (0, 3, 3),
            (3, 4, 7),
            (4, 0, 4),
        ];
        let adjacency_list = AdjacencyList::new_weighted_directed((n, m), &e);
        assert_eq!(adjacency_list.nodes_len(), 5);
        assert_eq!(
            adjacency_list.neighbors(0),
            &[1, 3].iter().cloned().collect()
        );
        assert_eq!(adjacency_list.neighbors(1), &[2].iter().cloned().collect());
        assert_eq!(adjacency_list.neighbors(2), &[0].iter().cloned().collect());
        assert_eq!(adjacency_list[3], [4].iter().cloned().collect());
        assert_eq!(adjacency_list[4], [0].iter().cloned().collect());
        assert_eq!(adjacency_list.weight(0, 1), 1);
        assert_eq!(adjacency_list.weight(3, 4), 7);
        assert_eq!(adjacency_list[(2, 0)], 2);
        assert_eq!(adjacency_list[(4, 0)], 4);
    }
}
