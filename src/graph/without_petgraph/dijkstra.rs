use super::structure::{AdjacencyList, Unweighted, Weighted};
use num::{Unsigned, Zero};

use std::{collections::BinaryHeap, ops::Add};

#[derive(PartialEq, Eq)]
pub struct MinHeapRecord<W: Ord> {
    weight: Option<W>,
    node_index: usize,
}
impl<W: Ord> Ord for MinHeapRecord<W> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // for min-heap, compare is reversed
        match (&self.weight, &other.weight) {
            (Some(a), Some(b)) => b.cmp(a),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => unreachable!(),
        }
    }
}
impl<W: Ord> PartialOrd for MinHeapRecord<W> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // for min-heap, compare is reversed
        Some(self.cmp(other))
    }
}

impl<D> AdjacencyList<Unweighted, D> {
    /// **O((n+m)log(n))**, shortest path number of hops and its route, with dijkstra algorithm
    pub fn dijkstra<T: Copy + Ord + Unsigned>(
        &self,
        start: usize,
        goal: usize,
    ) -> (Option<T>, Vec<usize>) {
        let n = self.nodes_len();
        if start >= n {
            return (None, Vec::new());
        }
        let (mut distance, mut prev) = (vec![None; n], vec![None; n]);
        distance[start] = Some(T::zero());
        let mut heap = BinaryHeap::new();
        heap.push(MinHeapRecord { weight: distance[start], node_index: start });
        while let Some(MinHeapRecord { weight, node_index: current }) = heap.pop() {
            if current == goal {
                let (mut visitor, mut predecessor) = (goal, Vec::new());
                while let Some(p) = prev[visitor] {
                    predecessor.push(visitor);
                    visitor = p;
                }
                predecessor.push(start);
                predecessor.reverse();
                return (weight, predecessor);
            }
            if distance[current] != None && weight > distance[current] {
                continue;
            }
            for &nei in self.neighbors(current) {
                let next_dist = distance[current].unwrap_or(T::zero()) + T::one();
                if distance[nei] == None || next_dist < distance[nei].unwrap_or(T::zero()) {
                    heap.push(MinHeapRecord { weight: Some(next_dist), node_index: nei });
                    distance[nei] = Some(next_dist);
                    prev[nei] = Some(current);
                }
            }
        }
        (None, Vec::new())
    }
}

impl<W: Copy + Add + Ord + Zero, D> AdjacencyList<Weighted<W>, D> {
    /// **O((n+m)log(n))**, shortest path distance and its route, with dijkstra algorithm
    pub fn dijkstra(&self, start: usize, goal: usize) -> (Option<W>, Vec<usize>) {
        let n = self.nodes_len();
        if start >= n {
            return (None, Vec::new());
        }
        let (mut distance, mut prev) = (vec![None; n], vec![None; n]);
        distance[start] = Some(W::zero());
        let mut heap = BinaryHeap::new();
        heap.push(MinHeapRecord { weight: distance[start], node_index: start });
        while let Some(MinHeapRecord { weight, node_index: current }) = heap.pop() {
            if current == goal {
                let (mut visitor, mut predecessor) = (goal, Vec::new());
                while let Some(p) = prev[visitor] {
                    predecessor.push(visitor);
                    visitor = p;
                }
                predecessor.push(start);
                predecessor.reverse();
                return (weight, predecessor);
            }
            if distance[current] != None && weight > distance[current] {
                continue;
            }
            for &nei in self.neighbors(current) {
                let next_dist = distance[current].unwrap_or(W::zero()) + self.weight(current, nei);
                if distance[nei] == None || next_dist < distance[nei].unwrap_or(W::zero()) {
                    heap.push(MinHeapRecord { weight: Some(next_dist), node_index: nei });
                    distance[nei] = Some(next_dist);
                    prev[nei] = Some(current);
                }
            }
        }
        (None, Vec::new())
    }
}

#[cfg(test)]
mod test {
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
        assert_eq!(adjacency_list.dijkstra(1, 3), (Some(2u64), vec![1, 0, 3]));
        assert_eq!(adjacency_list.dijkstra(1, 4), (Some(2u64), vec![1, 0, 4]));
        assert_eq!(adjacency_list.dijkstra(3, 4), (Some(1u64), vec![3, 4]));
        assert_eq!(adjacency_list.dijkstra::<u64>(1, 5), (None, vec![]));
        assert_eq!(adjacency_list.dijkstra::<u64>(5, 1), (None, vec![]));
    }

    #[test]
    fn test_unweighted_directed() {
        // 1-2
        // \ /
        //  0
        // / \
        // 3-4
        let (n, m) = (5, 6);
        let e = vec![(0, 1), (1, 2), (0, 2), (0, 3), (3, 4), (0, 4)];
        let adjacency_list = AdjacencyList::new_unweighted_directed((n, m), &e);
        assert_eq!(adjacency_list.dijkstra::<u64>(1, 3), (None, vec![]));
        assert_eq!(adjacency_list.dijkstra(0, 4), (Some(1u64), vec![0, 4]));
        assert_eq!(adjacency_list.dijkstra(3, 4), (Some(1u64), vec![3, 4]));
        assert_eq!(adjacency_list.dijkstra::<u64>(1, 5), (None, vec![]));
        assert_eq!(adjacency_list.dijkstra::<u64>(5, 1), (None, vec![]));
    }
    #[test]
    fn test_dijkstra_weighted_undirected() {
        //     3
        //    1-2
        //  1 \ / 2
        //     0
        //  1 / \ 2
        //    3-4
        //     7
        let (n, m) = (5, 6);
        let e = vec![(0, 1, 1), (1, 2, 3), (2, 0, 2), (0, 3, 1), (3, 4, 7), (4, 0, 2)];
        let adjacency_list = AdjacencyList::new_weighted_undirected((n, m), &e);
        assert_eq!(adjacency_list.dijkstra(1, 3), (Some(2), vec![1, 0, 3]));
        assert_eq!(adjacency_list.dijkstra(1, 4), (Some(3), vec![1, 0, 4]));
        assert_eq!(adjacency_list.dijkstra(3, 4), (Some(3), vec![3, 0, 4]));
        assert_eq!(adjacency_list.dijkstra(1, 5), (None, vec![]));
        assert_eq!(adjacency_list.dijkstra(5, 1), (None, vec![]));
    }

    #[test]
    fn test_weighted_directed() {
        //     3
        //    1-2
        //  1 \ / 5
        //     0
        //  3 / \ 4
        //    3-4
        //     7
        let (n, m) = (5, 6);
        let e = vec![(0, 1, 1), (1, 2, 3), (0, 2, 5), (0, 3, 3), (3, 4, 7), (0, 4, 4)];
        let adjacency_list = AdjacencyList::new_weighted_directed((n, m), &e);
        assert_eq!(adjacency_list.dijkstra(1, 3), (None, vec![]));
        assert_eq!(adjacency_list.dijkstra(0, 4), (Some(4u64), vec![0, 4]));
        assert_eq!(adjacency_list.dijkstra(0, 2), (Some(4u64), vec![0, 1, 2]));
        assert_eq!(adjacency_list.dijkstra(3, 4), (Some(7u64), vec![3, 4]));
        assert_eq!(adjacency_list.dijkstra(1, 5), (None, vec![]));
        assert_eq!(adjacency_list.dijkstra(5, 1), (None, vec![]));
    }
}
