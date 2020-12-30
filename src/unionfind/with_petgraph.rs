//! example of how to use petgraph::unionfind::Unionfind;
//! show test code from src

#[cfg(test)]
mod tests {
    use petgraph::unionfind::UnionFind;

    #[test]
    fn unionfind_test() {
        let mut forest = UnionFind::new(10usize);
        forest.union(0u32, 9);
        forest.union(5, 9);
        assert!(forest.equiv(0, 5));
        forest.union(3, 5);
        assert_eq!(forest.find(3), 0);
        assert_eq!(forest.into_labeling(), vec![0, 1, 2, 0, 4, 0, 6, 7, 8, 0]);
    }

    #[test]
    fn unionfind_test_from_ac() {
        // https://atcoder.jp/contests/atc001/tasks/unionfind_a
        let (n, _q) = (8, 9);
        let pab = vec![
            (0usize, 1usize, 2usize),
            (0, 3, 2),
            (1, 1, 3),
            (1, 1, 4),
            (0, 2, 4),
            (1, 4, 1),
            (0, 4, 2),
            (0, 0, 0),
            (1, 0, 0),
        ];
        let mut ans = [true, false, true, true].iter().cloned();

        let mut unionfind = UnionFind::new(n);
        for (pi, ai, bi) in pab {
            match pi {
                0 => {
                    unionfind.union(ai, bi);
                }
                1 => assert_eq!(unionfind.equiv(ai, bi), ans.next().unwrap()),
                _ => unreachable!(),
            }
        }
    }
}
