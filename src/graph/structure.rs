use std::{collections::HashMap, marker::PhantomData};

#[derive(Clone, Debug)]
pub enum Undirected {}

#[derive(Clone, Debug)]
pub enum Directed {}

/// graph
pub struct AdjacencyList<W, D> {
    list: Vec<Vec<usize>>,
    weight: HashMap<(usize, usize), W>,
    directed: PhantomData<D>,
}

/// unweighted undirected graph
impl AdjacencyList<(), Undirected> {
    /// **O(m)**, convert sequence of edges(unweighted, undirected) to adjacency list
    pub fn new_unweighted_undirected((n, _m): (usize, usize), e: &[(usize, usize)]) -> Self {
        let e: Vec<_> = e.iter().map(|&(u, v)| (u, v, ())).collect();
        Self::new_weighted_undirected((n, _m), &e)
    }
}

/// unweighted directed graph
impl AdjacencyList<(), Directed> {
    /// **O(m)**, convert sequence of edges(unweighted, undirected) to adjacency list
    pub fn new_unweighted_directed((n, _m): (usize, usize), e: &[(usize, usize)]) -> Self {
        let e: Vec<_> = e.iter().map(|&(u, v)| (u, v, ())).collect();
        Self::new_weighted_directed((n, _m), &e)
    }
}

/// weighted undirected graph
impl<W: Copy> AdjacencyList<W, Undirected> {
    /// **O(m)**, convert sequence of edges(weighted, undirected) to adjacency list
    pub fn new_weighted_undirected((n, _m): (usize, usize), e: &[(usize, usize, W)]) -> Self {
        let mut edges = vec![Vec::new(); n];
        let mut weight_map = HashMap::new();
        for &(u, v, w) in e {
            edges[u].push(v);
            edges[v].push(u);
            weight_map.insert((u, v), w);
            weight_map.insert((v, u), w);
        }
        Self {
            list: edges,
            weight: weight_map,
            directed: PhantomData,
        }
    }
}

/// weighted directed graph
impl<W: Copy> AdjacencyList<W, Directed> {
    /// **O(m)**, convert sequence of edges(weighted, undirected) to adjacency list
    pub fn new_weighted_directed((n, _m): (usize, usize), e: &[(usize, usize, W)]) -> Self {
        let mut edges = vec![Vec::new(); n];
        let mut weight_map = HashMap::new();
        for &(u, v, w) in e {
            edges[u].push(v);
            weight_map.insert((u, v), w);
        }
        Self {
            list: edges,
            weight: weight_map,
            directed: PhantomData,
        }
    }
}

/// weighted graph
impl<W: Copy, D> AdjacencyList<W, D> {
    /// **O(1)**, get weight of edge
    pub fn weight(&self, u: usize, v: usize) -> W {
        self.weight[&(u, v)]
    }
}

/// graph
impl<W, D> AdjacencyList<W, D> {
    /// **O(1)**, get node's neighbors list reference
    pub fn neighbors(&self, node: usize) -> &Vec<usize> {
        &self.list[node]
    }

    /// **O(1)**, number of nodes
    pub fn nodes(&self) -> usize {
        self.list.len()
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
        assert_eq!(adjacency_list.nodes(), 5);
        assert_eq!(adjacency_list.neighbors(0), &[1, 2, 3, 4]);
        assert_eq!(adjacency_list.neighbors(1), &[0, 2]);
        assert_eq!(adjacency_list.neighbors(2), &[0, 1]);
        assert_eq!(adjacency_list.neighbors(3), &[0, 4]);
        assert_eq!(adjacency_list.neighbors(4), &[0, 3]);
    }
    // #[test]
    // fn test_unweighted_directed() {
    //     // 1-2
    //     // \ /
    //     //  0
    //     // / \
    //     // 3-4
    //     let (n, m) = (5, 6);
    //     let e = vec![(0, 1), (1, 2), (2, 0), (0, 3), (3, 4), (4, 0)];
    //     let adjacency_list = AdjacencyList::new_directed((n, m), &e);
    //     assert_eq!(adjacency_list.len(), 5);
    //     assert_eq!(adjacency_list[&0], [1, 3].iter().cloned().collect());
    //     assert_eq!(adjacency_list[&1], [2].iter().cloned().collect());
    //     assert_eq!(adjacency_list[&2], [0].iter().cloned().collect());
    //     assert_eq!(adjacency_list[&3], [4].iter().cloned().collect());
    //     assert_eq!(adjacency_list[&4], [0].iter().cloned().collect());
    // }

    // #[test]
    // fn test_weighted_undirected() {
    //     //     3
    //     //    1-2
    //     //  1 \ / 2
    //     //     0
    //     //  3 / \ 4
    //     //    3-4
    //     //     7
    //     let (n, m) = (5, 6);
    //     let e = vec![
    //         (0, 1, 1),
    //         (1, 2, 3),
    //         (2, 0, 2),
    //         (0, 3, 3),
    //         (3, 4, 7),
    //         (4, 0, 4),
    //     ];
    //     let adjacency_list = weighted_undirected_to_adjacency_list((n, m), &e);
    //     assert_eq!(adjacency_list.len(), 5);
    //     assert_eq!(
    //         adjacency_list[&0],
    //         [(1, 1), (2, 2), (3, 3), (4, 4)].iter().cloned().collect()
    //     );
    //     assert_eq!(
    //         adjacency_list[&1],
    //         [(0, 1), (2, 3)].iter().cloned().collect()
    //     );
    //     assert_eq!(
    //         adjacency_list[&2],
    //         [(0, 2), (1, 3)].iter().cloned().collect()
    //     );
    //     assert_eq!(
    //         adjacency_list[&3],
    //         [(0, 3), (4, 7)].iter().cloned().collect()
    //     );
    //     assert_eq!(
    //         adjacency_list[&4],
    //         [(0, 4), (3, 7)].iter().cloned().collect()
    //     );
    // }

    // #[test]
    // fn test_weighted_directed() {
    //     //     3
    //     //    1-2
    //     //  1 \ / 2
    //     //     0
    //     //  3 / \ 4
    //     //    3-4
    //     //     7
    //     let (n, m) = (5, 6);
    //     let e = vec![
    //         (0, 1, 1),
    //         (1, 2, 3),
    //         (2, 0, 2),
    //         (0, 3, 3),
    //         (3, 4, 7),
    //         (4, 0, 4),
    //     ];
    //     let adjacency_list = weighted_directed_to_adjacency_list((n, m), &e);
    //     assert_eq!(adjacency_list.len(), 5);
    //     assert_eq!(
    //         adjacency_list[&0],
    //         [(1, 1), (3, 3)].iter().cloned().collect()
    //     );
    //     assert_eq!(adjacency_list[&1], [(2, 3)].iter().cloned().collect());
    //     assert_eq!(adjacency_list[&2], [(0, 2)].iter().cloned().collect());
    //     assert_eq!(adjacency_list[&3], [(4, 7)].iter().cloned().collect());
    //     assert_eq!(adjacency_list[&4], [(0, 4)].iter().cloned().collect());
    // }
}
