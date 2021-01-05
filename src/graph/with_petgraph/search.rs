#[cfg(test)]
mod tests {
    use petgraph::visit::{depth_first_search, Control, DfsEvent};
    use petgraph::visit::{Bfs, Dfs};
    use petgraph::{graph::UnGraph, Graph};

    #[test]
    fn bfs_order_unweighted_undirected() {
        // 1-2
        // \ /
        //  0
        // / \
        // 3-4
        let (_n, _m) = (5, 6);
        let e = vec![(0, 1), (1, 2), (2, 0), (0, 3), (3, 4), (4, 0)];
        let g = UnGraph::<(), ()>::from_edges(e);
        let mut bfs = Bfs::new(&g, 1.into());
        let mut bfs_order = Vec::new();
        while let Some(node) = bfs.next(&g) {
            bfs_order.push(node.index())
        }
        assert!(
            bfs_order == [1, 2, 0, 3, 4]
                || bfs_order == [1, 2, 0, 4, 3]
                || bfs_order == [1, 0, 2, 3, 4]
                || bfs_order == [1, 0, 2, 4, 3]
        );
    }

    #[test]
    fn dfs_order_unweighted_directed() {
        // 1-2
        // \ /
        //  0
        // / \
        // 3-4
        let (_n, _m) = (5, 6);
        let e = vec![(0, 1), (1, 2), (2, 0), (0, 3), (3, 4), (4, 0)];
        let g = Graph::<(), ()>::from_edges(&e);
        let mut dfs = Dfs::new(&g, 0.into());
        let mut dfs_order = Vec::new();
        while let Some(node) = dfs.next(&g) {
            dfs_order.push(node.index())
        }
        assert!(dfs_order == [0, 1, 2, 3, 4] || dfs_order == [0, 3, 4, 1, 2]);
    }

    #[test]
    fn dfs_weighted_undirected() {
        //     3
        //    1-2
        //  1 \ / 2
        //     0
        //  3 / \ 4
        //    3-4
        //     7
        let (_n, _m) = (5, 6);
        let e = vec![
            (0, 1, 1),
            (1, 2, 3),
            (2, 0, 2),
            (0, 3, 3),
            (3, 4, 7),
            (4, 0, 4),
        ];
        let g = Graph::<(), _>::from_edges(&e);
        let mut dfs = Dfs::new(&g, 0.into());
        let mut dfs_order = Vec::new();
        while let Some(node) = dfs.next(&g) {
            dfs_order.push(node.index())
        }
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
    fn bfs_weighted_directed() {
        //     3
        //    1-2
        //  1 \ / 2
        //     0
        //  3 / \ 4
        //    3-4
        //     7
        let (_n, _m) = (5, 6);
        let e = vec![
            (0, 1, 1),
            (1, 2, 3),
            (2, 0, 2),
            (0, 3, 3),
            (3, 4, 7),
            (4, 0, 4),
        ];
        let g = Graph::<(), _>::from_edges(&e);
        let mut bfs = Bfs::new(&g, 1.into());
        let mut bfs_order = Vec::new();
        while let Some(node) = bfs.next(&g) {
            bfs_order.push(node.index())
        }
        assert_eq!(bfs_order, [1, 2, 0, 3, 4]);
    }

    #[test]
    fn bfs_visited_weighted_directed() {
        //     3
        //    1-2
        //  1 \ / 2
        //     0
        //  3 / \ 4
        //    3-4
        //     7
        let (_n, _m) = (5, 6);
        let e = vec![
            (0, 1, 1),
            (1, 2, 3),
            (2, 0, 2),
            (0, 3, 3),
            (3, 4, 7),
            (4, 0, 4),
        ];
        let g = Graph::<(), _>::from_edges(&e);
        // https://docs.rs/petgraph/0.5.1/petgraph/visit/fn.depth_first_search.html
        let mut dfs_order = Vec::new();
        let start = 1.into();
        depth_first_search(&g, Some(start), |event| {
            match event {
                DfsEvent::Discover(node, _) => dfs_order.push(node.index()),
                DfsEvent::Finish(node, _) => {
                    dfs_order.push(node.index());
                    if node == start {
                        return Control::Break(node);
                    }
                }
                _ => (),
            }
            Control::Continue
        });
        assert_eq!(dfs_order, [1, 2, 0, 3, 4, 4, 3, 0, 2, 1]);
    }
}
