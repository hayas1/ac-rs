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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compress() {
        assert!(!is_bipartite(3, &[(0, 1), (0, 2), (1, 2)]));
        assert!(is_bipartite(5, &[(0, 1), (0, 2), (2, 3), (3, 4), (0, 4)]));
        assert!(!is_bipartite(5, &[(0, 1), (0, 2), (2, 3), (3, 4), (0, 3)]));
    }
}
