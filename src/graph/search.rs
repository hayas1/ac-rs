use std::collections::{HashSet, VecDeque};

use crate::graph::structure::AdjacencyList;

/// **O(n)**, return breadth first search order
pub fn bfs<W, D>(graph: AdjacencyList<W, D>, start: usize) -> Vec<usize> {
    let mut queue = VecDeque::new();
    let mut touched = HashSet::new();
    let mut order = Vec::new();
    queue.push_back(start);
    touched.insert(start);
    while let Some(node) = queue.pop_front() {
        order.push(node);
        let neighbors = &graph[node];
        for &nei in neighbors {
            queue.push_back(nei);
            touched.insert(nei);
        }
    }
    order
}

/// **O(n)**, return depth first search order
pub fn dfs<W, D>(graph: AdjacencyList<W, D>, start: usize) -> Vec<usize> {
    let mut stack = Vec::new();
    let mut touched = HashSet::new();
    let mut order = Vec::new();
    stack.push(start);
    touched.insert(start);
    while let Some(node) = stack.pop() {
        order.push(node);
        let neighbors = &graph[node];
        for &nei in neighbors {
            stack.push(nei);
            touched.insert(nei);
        }
    }
    order
}
