use num::Unsigned;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    ops::{Add, Bound, RangeBounds},
};

pub fn from_to_indices<N, R, Nei>(neighbors: &HashMap<N, Vec<Nei>>, from_to: R) -> (N, Option<N>)
where
    N: Clone + Ord + Eq + Hash,
    R: RangeBounds<N>,
{
    let from = match from_to.start_bound() {
        Bound::Unbounded => panic!("from node must be specified."),
        Bound::Excluded(l) => l.clone(),
        Bound::Included(l) => l.clone(),
    };
    if !neighbors.contains_key(&from) {
        panic!("from node must have outgoing edge.");
    }
    let to = match from_to.end_bound() {
        Bound::Unbounded => None,
        Bound::Excluded(r) => Some(r.clone()),
        Bound::Included(r) => Some(r.clone()),
    };
    (from, to)
}

/// **O((n+m)log(n))**, shortest paths cost, with dijkstra algorithm
pub fn dijkstra<N, R, W, C, F>(
    neighbors: &HashMap<N, Vec<(N, W)>>,
    from_to: R,
    next_cost: F,
) -> HashMap<N, C>
where
    N: Clone + Ord + Eq + Hash,
    R: RangeBounds<N>,
    C: Clone + Add + Ord + Unsigned,
    F: Fn(C, &W) -> C,
{
    let (from, to) = from_to_indices(neighbors, from_to);
    let mut costs: HashMap<_, _> = vec![(from.clone(), C::zero())].into_iter().collect();
    let mut heap: BinaryHeap<_> = vec![Reverse((C::zero(), from))].into_iter().collect(); // for min-heap, use Reverse
    while let Some(Reverse((cost, node))) = heap.pop() {
        if Some(node.clone()) == to {
            return vec![(node, cost)].into_iter().collect(); // when first come to the goal, the cost at time is minimum.
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
    costs // if loop finish, all node in connected components is visited
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
        assert_eq!(dijkstra(&neighbors, 1..3, |c, w| c + w), vec![(3, 2)].into_iter().collect());
        assert_eq!(
            dijkstra(&neighbors, 1.., |c, w| c + w),
            vec![(0, 1), (1, 0), (2, 1), (3, 2), (4, 2)].into_iter().collect()
        );
        assert_eq!(dijkstra(&neighbors, 3..=4, |c: u64, w| c + w)[&4], 1);
        // node 5 does not exist, so dijkstra search all node in the connected component
        assert_eq!(
            dijkstra(&neighbors, 1..5, |c, w| c + w),
            vec![(0, 1), (1, 0), (2, 1), (3, 2), (4, 2)].into_iter().collect()
        );
        {
            // silent output
            let prev_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(|_| {}));
            let result = std::panic::catch_unwind(|| dijkstra(&neighbors, 5..1, |c: u64, w| c + w));
            assert!(result.is_err()); // node 5 does not exist, so dijkstra algorithm cannot start
            std::panic::set_hook(prev_hook);
        }
    }
}
