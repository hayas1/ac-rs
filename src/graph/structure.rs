use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

/// **O(m)**, convert sequence of edges(unweighted, undirected) to adjacency list
pub fn unweighted_undirected_to_adjacency_list(
    (n, _m): (usize, usize),
    e: &[(usize, usize)],
) -> HashMap<usize, HashSet<usize>> {
    let mut edges: HashMap<_, _> = (0..n).map(|u| (u, HashSet::new())).collect();
    for &(u, v) in e {
        edges
            .get_mut(&u)
            .unwrap_or_else(|| panic!("unexpected node: {}", u)) // expect will cost format every time
            .insert(v);
        edges
            .get_mut(&v)
            .unwrap_or_else(|| panic!("unexpected node: {}", u)) // expect will cost format every time
            .insert(u);
    }
    edges
}

/// **O(m)**, convert sequence of edges(unweighted, directed) to adjacency list
pub fn unweighted_directed_to_adjacency_list(
    (n, _m): (usize, usize),
    e: &[(usize, usize)],
) -> HashMap<usize, HashSet<usize>> {
    let mut edges: HashMap<_, _> = (0..n).map(|u| (u, HashSet::new())).collect();
    for &(u, v) in e {
        edges
            .get_mut(&u)
            .unwrap_or_else(|| panic!("unexpected node: {}", u)) // expect will cost format every time
            .insert(v);
    }
    edges
}

/// **O(m)**, convert sequence of edges(weighted, undirected) to adjacency list
pub fn weighted_undirected_to_adjacency_list<W: Copy + Hash + Eq>(
    (n, _m): (usize, usize),
    e: &[(usize, usize, W)],
) -> HashMap<usize, HashSet<(usize, W)>> {
    let mut edges: HashMap<_, _> = (0..n).map(|u| (u, HashSet::new())).collect();
    for &(u, v, w) in e {
        edges
            .get_mut(&u)
            .unwrap_or_else(|| panic!("unexpected node: {}", u)) // expect will cost format every time
            .insert((v, w));
        edges
            .get_mut(&v)
            .unwrap_or_else(|| panic!("unexpected node: {}", v)) // expect will cost format every time
            .insert((u, w));
    }
    edges
}

/// **O(m)**, convert sequence of edges(weighted, directed) to adjacency list
pub fn weighted_directed_to_adjacency_list<W: Copy + Hash + Eq>(
    (n, _m): (usize, usize),
    e: &[(usize, usize, W)],
) -> HashMap<usize, HashSet<(usize, W)>> {
    let mut edges: HashMap<_, _> = (0..n).map(|u| (u, HashSet::new())).collect();
    for &(u, v, w) in e {
        edges
            .get_mut(&u)
            .unwrap_or_else(|| panic!("unexpected node: {}", u)) // expect will cost format every time
            .insert((v, w));
    }
    edges
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
        let adjacency_list = unweighted_undirected_to_adjacency_list((n, m), &e);
        assert_eq!(adjacency_list.len(), 5);
        assert_eq!(adjacency_list[&0], [1, 2, 3, 4].iter().cloned().collect());
        assert_eq!(adjacency_list[&1], [0, 2].iter().cloned().collect());
        assert_eq!(adjacency_list[&2], [0, 1].iter().cloned().collect());
        assert_eq!(adjacency_list[&3], [0, 4].iter().cloned().collect());
        assert_eq!(adjacency_list[&4], [0, 3].iter().cloned().collect());
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
        let adjacency_list = unweighted_directed_to_adjacency_list((n, m), &e);
        assert_eq!(adjacency_list.len(), 5);
        assert_eq!(adjacency_list[&0], [1, 3].iter().cloned().collect());
        assert_eq!(adjacency_list[&1], [2].iter().cloned().collect());
        assert_eq!(adjacency_list[&2], [0].iter().cloned().collect());
        assert_eq!(adjacency_list[&3], [4].iter().cloned().collect());
        assert_eq!(adjacency_list[&4], [0].iter().cloned().collect());
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
        let adjacency_list = weighted_undirected_to_adjacency_list((n, m), &e);
        assert_eq!(adjacency_list.len(), 5);
        assert_eq!(
            adjacency_list[&0],
            [(1, 1), (2, 2), (3, 3), (4, 4)].iter().cloned().collect()
        );
        assert_eq!(
            adjacency_list[&1],
            [(0, 1), (2, 3)].iter().cloned().collect()
        );
        assert_eq!(
            adjacency_list[&2],
            [(0, 2), (1, 3)].iter().cloned().collect()
        );
        assert_eq!(
            adjacency_list[&3],
            [(0, 3), (4, 7)].iter().cloned().collect()
        );
        assert_eq!(
            adjacency_list[&4],
            [(0, 4), (3, 7)].iter().cloned().collect()
        );
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
        let adjacency_list = weighted_directed_to_adjacency_list((n, m), &e);
        assert_eq!(adjacency_list.len(), 5);
        assert_eq!(
            adjacency_list[&0],
            [(1, 1), (3, 3)].iter().cloned().collect()
        );
        assert_eq!(adjacency_list[&1], [(2, 3)].iter().cloned().collect());
        assert_eq!(adjacency_list[&2], [(0, 2)].iter().cloned().collect());
        assert_eq!(adjacency_list[&3], [(4, 7)].iter().cloned().collect());
        assert_eq!(adjacency_list[&4], [(0, 4)].iter().cloned().collect());
    }
}
