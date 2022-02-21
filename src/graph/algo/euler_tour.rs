use std::collections::{HashMap, HashSet};
type NodeId = usize;
/// **O(n+m)**, get visited time and left time by dfs
pub fn euler_tour(
    outgoings: &HashMap<NodeId, HashSet<NodeId>>,
    start: NodeId,
) -> (Vec<usize>, Vec<usize>) {
    fn euler_tour_recursive(
        outgoings: &HashMap<NodeId, HashSet<NodeId>>,
        node: NodeId,
        time: &mut usize,
        visit: &mut Vec<usize>,
        leave: &mut Vec<usize>,
        visited: &mut Vec<bool>,
    ) {
        visited[node] = true;
        visit[node] = *time;
        for &next in &outgoings[&node] {
            if !visited[next] {
                *time += 1;
                euler_tour_recursive(&outgoings, next, time, visit, leave, visited);
            }
        }
        *time += 1;
        leave[node] = *time;
    }
    let mut visited = vec![false; outgoings.len()];
    let mut time = 0;
    let (mut visit, mut leave) = (vec![0; outgoings.len()], vec![0; outgoings.len()]);
    euler_tour_recursive(&outgoings, start, &mut time, &mut visit, &mut leave, &mut visited);
    (visit, leave)
}

#[cfg(test)]
mod tests {

    use super::super::undirected_neighbors;
    use super::*;

    #[test]
    fn test_euler_tour_unweighted_undirected() {
        //    0
        //    |
        //    1
        //   / \
        // 3-2 4-5
        let e = vec![(0, 1), (1, 2), (2, 3), (1, 4), (4, 5)];
        let outgoings = undirected_neighbors(6, &e);
        let (visit, leave) = euler_tour(&outgoings, 0);
        assert!(
            (&visit, &leave) == (&vec![0, 1, 2, 3, 6, 7], &vec![11, 10, 5, 4, 9, 8])
                || (&visit, &leave) == (&vec![0, 1, 6, 7, 2, 3], &vec![11, 10, 9, 8, 5, 4])
        );
    }
}
