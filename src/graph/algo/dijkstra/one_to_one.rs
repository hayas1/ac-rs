use num::Unsigned;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    ops::Add,
};

/// **O((n+m)log(n))**, shortest path cost, with dijkstra algorithm
pub fn dijkstra<N, W, C, F>(
    neighbors: &HashMap<N, Vec<(N, W)>>,
    start: N,
    end: N,
    next_cost: F,
) -> Option<C>
where
    N: Clone + Ord + Eq + Hash,
    C: Clone + Add + Ord + Unsigned,
    F: Fn(C, &W) -> C,
{
    let mut costs: HashMap<_, _> = vec![(start.clone(), C::zero())].into_iter().collect();
    let mut heap: BinaryHeap<_> = vec![Reverse((C::zero(), start))].into_iter().collect(); // for min-heap, use Reverse
    while let Some(Reverse((cost, node))) = heap.pop() {
        if node == end {
            return Some(cost); // when first come to the goal, the cost at time is minimum.
        } else if costs.get(&node).and_then(|c| Some(cost > c.clone())).unwrap_or(false) {
            continue; // cost is pseudo-initialized by infinite, if node has larger cost, it is skipped
        }
        for (nei, weight) in neighbors.get(&node).unwrap_or(&Vec::new()) {
            let next_cost = next_cost(cost.clone(), &weight); // normal: next_cost(c, w) = c + w
            if costs.get(nei).and_then(|c| Some(next_cost < c.clone())).unwrap_or(true) {
                // cost is pseudo-initialized by infinite, if node has smaller cost, it is added to min-heap
                heap.push(Reverse((next_cost.clone(), nei.clone())));
                costs.insert(nei.clone(), next_cost);
            }
        }
    }
    None
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
        let (_n, _m) = (5, 6);
        let e = vec![(0, 1), (1, 2), (2, 0), (0, 3), (3, 4), (4, 0)];
        let mut neighbors = HashMap::new();
        for (u, v) in e {
            neighbors.entry(u).or_insert(Vec::new()).push((v, 1u64));
            neighbors.entry(v).or_insert(Vec::new()).push((u, 1u64));
        }
        assert_eq!(dijkstra(&neighbors, 1, 3, |c, w| c + w), Some(2));
        assert_eq!(dijkstra(&neighbors, 1, 4, |c, w| c + w), Some(2));
        assert_eq!(dijkstra(&neighbors, 3, 4, |c, w| c + w), Some(1));
        assert_eq!(dijkstra(&neighbors, 1, 5, |c: u64, w| c + w), None);
        assert_eq!(dijkstra(&neighbors, 5, 1, |c: u64, w| c + w), None);
    }

    #[test]
    fn test_unweighted_directed() {
        // 1-2
        // \ /
        //  0
        // / \
        // 3-4
        let (_n, _m) = (5, 6);
        let e = vec![(0, 1), (1, 2), (0, 2), (0, 3), (3, 4), (0, 4)];
        let mut neighbors = HashMap::new();
        for (u, v) in e {
            neighbors.entry(u).or_insert(Vec::new()).push((v, 1u64));
        }
        assert_eq!(dijkstra::<_, _, u64, _>(&neighbors, 1, 3, |c, w| c + w), None);
        assert_eq!(dijkstra(&neighbors, 0, 4, |c, w| c + w), Some(1));
        assert_eq!(dijkstra(&neighbors, 3, 4, |c, w| c + w), Some(1));
        assert_eq!(dijkstra(&neighbors, 1, 5, |c: u64, w| c + w), None);
        assert_eq!(dijkstra(&neighbors, 5, 1, |c: u64, w| c + w), None);
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
        let (_n, _m) = (5, 6);
        let e = vec![(0, 1, 1), (1, 2, 3), (2, 0, 2), (0, 3, 1), (3, 4, 7), (4, 0, 2)];
        let mut neighbors = HashMap::new();
        for (u, v, w) in e {
            neighbors.entry(u).or_insert(Vec::new()).push((v, w));
            neighbors.entry(v).or_insert(Vec::new()).push((u, w));
        }
        assert_eq!(dijkstra(&neighbors, 1, 3, |c, w| c + w), Some(2));
        assert_eq!(dijkstra(&neighbors, 1, 4, |c, w| c + w), Some(3));
        assert_eq!(dijkstra(&neighbors, 3, 4, |c, w| c + w), Some(3));
        assert_eq!(dijkstra(&neighbors, 1, 5, |c: u64, w| c + w), None);
        assert_eq!(dijkstra(&neighbors, 5, 1, |c: u64, w| c + w), None);
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
        let (_n, _m) = (5, 6);
        let e = vec![(0, 1, 1), (1, 2, 3), (0, 2, 5), (0, 3, 3), (3, 4, 7), (0, 4, 4)];
        let mut neighbors = HashMap::new();
        for (u, v, w) in e {
            neighbors.entry(u).or_insert(Vec::new()).push((v, w));
        }
        assert_eq!(dijkstra(&neighbors, 1, 3, |c: u64, w| c + w), None);
        assert_eq!(dijkstra(&neighbors, 0, 4, |c, w| c + w), Some(4));
        assert_eq!(dijkstra(&neighbors, 0, 2, |c, w| c + w), Some(4));
        assert_eq!(dijkstra(&neighbors, 3, 4, |c, w| c + w), Some(7));
        assert_eq!(dijkstra(&neighbors, 1, 5, |c: u64, w| c + w), None);
        assert_eq!(dijkstra(&neighbors, 5, 1, |c: u64, w| c + w), None);
    }
}
