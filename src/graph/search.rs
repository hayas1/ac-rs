use std::collections::{HashSet, VecDeque};

use crate::graph::structure::AdjacencyList;

// graph structure
impl<W, D> AdjacencyList<W, D> {
    /// **O(n)**, return breadth first search order
    pub fn bfs(&self, start: usize) -> Vec<usize> {
        let (mut queue, mut touched) = (VecDeque::new(), HashSet::new());
        let mut order = Vec::new();
        queue.push_back(start);
        touched.insert(start);
        while let Some(node) = queue.pop_front() {
            let neighbors = self.neighbors(node);
            for &nei in neighbors {
                if touched.contains(&nei) {
                    continue;
                }
                queue.push_back(nei);
                touched.insert(nei);
            }
            // here, some kind of processing on node
            order.push(node);
        }
        order
    }

    /// **O(n)**, return depth first search order
    pub fn dfs(&self, start: usize) -> Vec<usize> {
        fn dfs_recursive<W, D>(
            adjacency_list: &AdjacencyList<W, D>,
            node: usize,
            visited: &mut HashSet<usize>,
            order: &mut Vec<usize>,
        ) {
            visited.insert(node);
            let neighbors = adjacency_list.neighbors(node);
            // here, some kind of processing on node, before visit children
            order.push(node);
            for &nei in neighbors {
                if visited.contains(&nei) {
                    continue;
                }
                dfs_recursive(adjacency_list, nei, visited, order);
            }
            // here, some kind of processing on node, after visit children
        }
        let mut visited = HashSet::new();
        let mut order = Vec::new();
        dfs_recursive(self, start, &mut visited, &mut order);
        order
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bfs_unweighted_undirected() {
        // 1-2
        // \ /
        //  0
        // / \
        // 3-4
        let (n, m) = (5, 6);
        let e = vec![(0, 1), (1, 2), (2, 0), (0, 3), (3, 4), (4, 0)];
        let adjacency_list = AdjacencyList::new_unweighted_undirected((n, m), &e);
        let bfs_order = adjacency_list.bfs(1);
        assert!(
            bfs_order == [1, 2, 0, 3, 4]
                || bfs_order == [1, 2, 0, 4, 3]
                || bfs_order == [1, 0, 2, 3, 4]
                || bfs_order == [1, 0, 2, 4, 3]
        );
    }

    #[test]
    fn test_dfs_unweighted_directed() {
        // 1-2
        // \ /
        //  0
        // / \
        // 3-4
        let (n, m) = (5, 6);
        let e = vec![(0, 1), (1, 2), (2, 0), (0, 3), (3, 4), (4, 0)];
        let adjacency_list = AdjacencyList::new_unweighted_directed((n, m), &e);
        let dfs_order = adjacency_list.dfs(0);
        assert!(dfs_order == [0, 1, 2, 3, 4] || dfs_order == [0, 3, 4, 1, 2]);
    }

    #[test]
    fn test_dfs_weighted_undirected() {
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
        let dfs_order = adjacency_list.dfs(0);
        assert!(
            dfs_order == [0, 1, 2, 3, 4]
                || dfs_order == [0, 1, 2, 4, 3]
                || dfs_order == [0, 2, 1, 3, 4]
                || dfs_order == [0, 2, 1, 4, 3]
                || dfs_order == [0, 3, 4, 1, 2]
                || dfs_order == [0, 3, 4, 2, 1]
                || dfs_order == [0, 4, 3, 1, 2]
                || dfs_order == [0, 4, 3, 2, 1]
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
        let adjacency_list = AdjacencyList::new_weighted_directed((n, m), &e);
        let bfs_order = adjacency_list.bfs(1);
        assert_eq!(bfs_order, [1, 2, 0, 3, 4]);
    }
}
