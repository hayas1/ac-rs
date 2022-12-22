use petgraph::unionfind::UnionFind;

/// ***O(m log(n))***, check bipartite graph (warning: expect simple connected undirected graph)
pub fn is_bipartite(n: usize, e: &[(usize, usize)]) -> bool {
    let mut uf = UnionFind::new(2 * n);
    for &(u, v) in e {
        uf.union(u, n + v);
        uf.union(n + u, v);
    }
    (0..n).all(|v| uf.find(v) != uf.find(v + n))
}
