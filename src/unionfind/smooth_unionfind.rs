#![allow(dead_code)]

struct UnionFind {
    parents: Vec<usize>,
}
impl UnionFind {
    /// **O(n)** create n trees with themselves as roots
    fn new(n: usize) -> Self {
        UnionFind {
            parents: (0..n).collect(),
        }
    }

    /// **O(log(n))** marge 2 trees with primary and standby roots
    fn union(&mut self, primary: usize, standby: usize) -> bool {
        let primary_root = self.find(primary);
        let standby_root = self.find(standby);
        if primary_root != standby_root {
            self.parents[standby_root] = primary_root;
        }
        primary_root != standby_root
    }

    /// **O(log(n))** find the root of x, and update the roots of intermediate nodes
    fn find(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        } else {
            let root = self.find(self.parents[x]);
            self.parents[x] = root;
            root
        }
    }

    /// **O(log(n))** check does x and y belong same root
    fn equiv(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
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
