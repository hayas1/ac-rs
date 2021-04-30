use petgraph::prelude::*;
use petgraph::visit::{depth_first_search, DfsEvent};

/// **O(n)**, return diameter of given tree
pub fn diameter_of_tree<N, E>(graph: &UnGraph<N, E>) -> usize {
    let n = graph.node_count();
    let mut depth1 = vec![0; n];
    depth_first_search(
        graph,
        Some(graph.node_indices().next().expect("empty graph is invalid")),
        |event| {
            if let DfsEvent::TreeEdge(u, v) = event {
                depth1[v.index()] = depth1[u.index()];
            }
        },
    );
    let (leaf_index, _leaf_depth) =
        depth1.iter().enumerate().max_by_key(|(_, &d)| d).expect("empty graph is invalid.");
    let mut depth2 = vec![0; n];
    depth_first_search(graph, Some((leaf_index as u32).into()), |event| {
        if let DfsEvent::TreeEdge(u, v) = event {
            depth2[v.index()] = depth2[u.index()] + 1;
        }
    });
    *depth2.iter().max().expect("empty graph is invalid.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_diameter_test() {
        // 1 2
        // \ /
        //  0
        // /
        // 3-4
        let (_n, _m) = (5, 6);
        let e = vec![(0, 1), (2, 0), (0, 3), (3, 4)];
        let g = UnGraph::<(), ()>::from_edges(e);
        assert_eq!(diameter_of_tree(&g), 3);
    }

    #[test]
    #[should_panic]
    fn empty_tree_test() {
        let g: UnGraph<(), ()> = UnGraph::new_undirected();
        diameter_of_tree(&g);
    }

    #[test]
    fn one_node_tree_test() {
        let mut g: UnGraph<_, ()> = UnGraph::new_undirected();
        g.add_node(());
        assert_eq!(diameter_of_tree(&g), 0);
    }
}
