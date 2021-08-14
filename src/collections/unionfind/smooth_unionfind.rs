pub struct UnionFind {
    parents: Vec<usize>,
    size: Vec<usize>,
}
impl UnionFind {
    /// **O(n)**, create n trees with themselves as roots
    pub fn new(n: usize) -> Self {
        UnionFind { parents: (0..n).collect(), size: vec![1; n] }
    }

    /// **O(log(n))**, marge 2 trees with primary and standby roots, if union return true
    pub fn union(&mut self, primary: usize, standby: usize) -> bool {
        let primary_root = self.find(primary);
        let standby_root = self.find(standby);
        if primary_root != standby_root {
            self.parents[standby_root] = primary_root;
            self.size[primary_root] += self.size[standby_root];
        }
        primary_root != standby_root
    }

    /// **O(log(n))**, find the root of x, and update the roots of intermediate nodes
    pub fn find(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        } else {
            let root = self.find(self.parents[x]);
            self.parents[x] = root;
            root
        }
    }

    /// **O(log(n))**, get root of x, this method is immutable
    pub fn root(&self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        } else {
            self.root(self.parents[x])
        }
    }

    /// **O(log(n))**, check does x and y belong same root
    pub fn equiv(&self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    /// **O(log(n))**, return size of connected component
    pub fn size(&self, x: usize) -> usize {
        let root = self.root(x);
        self.size[root]
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
        assert_eq!(forest.size(0), 1);
        assert_eq!(forest.size(1), 3);
    }

    #[test]
    fn union2() {
        let mut forest = UnionFind::new(5);
        forest.union(2, 3);
        forest.union(1, 2);
        assert_eq!(forest.parents, vec![0, 1, 1, 2, 4]);
        assert_eq!((0..5).map(|i| forest.find(i)).collect::<Vec<_>>(), vec![0, 1, 1, 1, 4]);
    }

    #[test]
    fn find() {
        let mut forest = UnionFind::new(5);
        forest.union(0, 2);
        forest.union(2, 3);
        assert_eq!(forest.root(3), 0);
        assert_eq!(forest.find(3), 0);
        assert_eq!(forest.root(3), 0);
    }
}
