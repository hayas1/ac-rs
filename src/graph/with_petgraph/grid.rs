use std::collections::HashMap;

use petgraph::graph::NodeIndex;
use petgraph::Graph;

/// **O(hw)**, make directed grid like graph from map ('.' is load, '#' is wall)
pub fn field_to_directed_grid(
    (h, w): (usize, usize),
    field: &[Vec<char>],
) -> (Graph<(), usize>, HashMap<(usize, usize), NodeIndex>) {
    let mut g = Graph::new();
    let mut nodes = HashMap::new();
    // node construct in graph
    for (i, r) in field.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            match c {
                '.' => {
                    nodes.insert((i, j), g.add_node(()));
                }
                '#' => continue,
                _ => unreachable!(),
            }
        }
    }
    // edge construct in graph
    for (i, r) in field.iter().enumerate() {
        for (j, &c) in r.iter().enumerate() {
            if c == '#' {
                continue;
            }
            if i > 0 && field[i - 1][j] != '#' {
                g.add_edge(nodes[&(i, j)], nodes[&(i - 1, j)], 1);
            }
            if i < h - 1 && field[i + 1][j] != '#' {
                g.add_edge(nodes[&(i, j)], nodes[&(i + 1, j)], 1);
            }
            if j > 0 && field[i][j - 1] != '#' {
                g.add_edge(nodes[&(i, j)], nodes[&(i, j - 1)], 1);
            }
            if j < w - 1 && field[i][j + 1] != '#' {
                g.add_edge(nodes[&(i, j)], nodes[&(i, j + 1)], 1);
            }
        }
    }
    (g, nodes)
}

#[cfg(test)]
mod tests {
    use petgraph::algo::dijkstra;

    use super::*;

    #[test]
    fn grid_shortest_path_test1() {
        let (h, w) = (2, 3);
        let field = vec![
            ".##".chars().collect::<Vec<_>>(),
            "...".chars().collect::<Vec<_>>(),
        ];
        let (g, nodes) = field_to_directed_grid((h, w), &field);
        assert_eq!(
            dijkstra(&g, nodes[&(0, 0)], None, |e| *e.weight())[&nodes[&(1, 2)]],
            3
        );
    }

    #[test]
    fn grid_shortest_path_test2() {
        let (h, w) = (5, 5);
        let field = vec![
            ".####".chars().collect::<Vec<_>>(),
            ".....".chars().collect::<Vec<_>>(),
            ".###.".chars().collect::<Vec<_>>(),
            ".#.#.".chars().collect::<Vec<_>>(),
            ".##..".chars().collect::<Vec<_>>(),
        ];
        let (g, nodes) = field_to_directed_grid((h, w), &field);
        assert_eq!(
            dijkstra(&g, nodes[&(0, 0)], None, |e| *e.weight())[&nodes[&(4, 4)]],
            8
        );
        assert_eq!(
            dijkstra(&g, nodes[&(0, 0)], None, |e| *e.weight())[&nodes[&(4, 3)]],
            9
        );
        assert_eq!(
            dijkstra(&g, nodes[&(0, 0)], None, |e| *e.weight()).get(&nodes[&(3, 2)]),
            None
        );
    }
}
