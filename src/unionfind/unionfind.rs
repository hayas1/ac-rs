#![allow(dead_code)]

struct UnionFind {
    parents: Vec<usize>,
}
impl UnionFind {
    /// O(n) # create n trees with themselves as roots
    fn new(n: usize) -> Self {
        UnionFind {
            parents: (0..n).collect(),
        }
    }

    /// worst: O(n) # marge 2 trees with primary and standby roots
    fn union(&mut self, primary: usize, standby: usize) -> usize {
        let primary_root = self.find(primary);
        let standby_root = self.find(standby);
        self.parents[standby_root] = primary_root;
        primary_root
    }

    /// worst: O(n) # find the root of x
    fn find(&self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        } else {
            self.find(self.parents[x])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn union() {
        let mut forest = UnionFind::new(5);
        forest.union(1, 2);
        forest.union(2, 3);
        assert_eq!(forest.parents, vec![0, 1, 1, 1, 4]);
    }

    #[test]
    fn union2() {
        let mut forest = UnionFind::new(5);
        forest.union(2, 3);
        forest.union(1, 2);
        assert_eq!(forest.parents, vec![0, 1, 1, 2, 4]);
        assert_eq!(
            (0..5).map(|i| forest.find(i)).collect::<Vec<_>>(),
            vec![0, 1, 1, 1, 4]
        );
    }

    #[test]
    fn find() {
        let mut forest = UnionFind::new(5);
        forest.union(0, 2);
        forest.union(2, 3);
        assert_eq!(forest.find(3), 0);
    }
}
