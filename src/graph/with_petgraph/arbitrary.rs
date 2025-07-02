//! example of how to use petgraph::Graph::from_edges
//!
//! show test code from src

#[cfg(test)]
mod tests {
    use petgraph::algo::dijkstra;
    use petgraph::graph::{NodeIndex, UnGraph};

    // use super::*;

    #[test]
    fn test_zero_index() {
        // 1-2
        // \ /
        //  0
        // / \
        // 3-4
        let (n, m) = (5, 6);
        let e = vec![(0, 1), (1, 2), (2, 0), (0, 3), (3, 4), (4, 0)];
        let g = UnGraph::<(), ()>::from_edges(&e);
        assert_eq!(g.node_count(), n);
        assert_eq!(g.edge_count(), m);
        assert_eq!(
            dijkstra(&g, 0.into(), None, |_| 1),
            vec![(0, 0), (1, 1), (2, 1), (3, 1), (4, 1)]
                .iter()
                .map(|&(k, v)| (k.into(), v))
                .collect()
        );
        assert_eq!(dijkstra(&g, 1.into(), Some(4.into()), |_| 1)[&NodeIndex::from(4)], 2);
        assert_eq!(dijkstra(&g, 1.into(), Some(4.into()), |_| 2)[&NodeIndex::from(4)], 4);
    }

    #[test]
    fn test_one_index() {
        // 2-3
        // \ /
        //  1    0
        // / \
        // 4-5
        let (n, m) = (5, 6);
        let e = vec![(1, 2), (2, 3), (3, 1), (1, 4), (4, 5), (5, 1)];
        let g = UnGraph::<(), ()>::from_edges(&e);
        assert_eq!(g.node_count(), n + 1);
        assert_eq!(g.edge_count(), m);
        assert_eq!(
            dijkstra(&g, 1.into(), None, |_| 1),
            vec![(1, 0), (2, 1), (3, 1), (4, 1), (5, 1)]
                .iter()
                .map(|&(k, v)| (k.into(), v))
                .collect()
        );
        assert_eq!(dijkstra(&g, 2.into(), Some(5.into()), |_| 1)[&NodeIndex::from(5)], 2);
        assert_eq!(dijkstra(&g, 2.into(), Some(5.into()), |_| 2)[&NodeIndex::from(5)], 4);
        // println!(
        //     "{:?}",
        //     petgraph::dot::Dot::with_config(&g, &[petgraph::dot::Config::EdgeNoLabel])
        // );
    }

    #[test]
    fn test_zero_index_with_weight() {
        //     3
        //    1-2
        //  1 \ / 2
        //     0
        //  3 / \ 4
        //    3-4
        //     7
        let (n, m) = (5, 6);
        let e = vec![(0, 1, 1), (1, 2, 3), (2, 0, 2), (0, 3, 3), (3, 4, 7), (4, 0, 4)];
        let g = UnGraph::<(), usize>::from_edges(&e);
        assert_eq!(g.node_count(), n);
        assert_eq!(g.edge_count(), m);
        assert_eq!(
            dijkstra(&g, 0.into(), None, |e| *e.weight()),
            vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)]
                .iter()
                .map(|&(k, v)| (k.into(), v))
                .collect()
        );
        assert_eq!(dijkstra(&g, 1.into(), Some(4.into()), |_| 1)[&NodeIndex::from(4)], 2);
        assert_eq!(dijkstra(&g, 1.into(), Some(4.into()), |e| *e.weight())[&NodeIndex::from(4)], 5);
    }
}
