use std::collections::{HashMap, HashSet};

pub struct MergeTechnique {
    parents: Vec<usize>,
    members: HashMap<usize, HashSet<usize>>,
}
impl MergeTechnique {
    /// **O(n)**, create n trees with themselves as roots
    pub fn new(n: usize) -> Self {
        let parents = (0..n).collect();
        let members = (0..n).map(|i| (i, vec![i].into_iter().collect())).collect();
        MergeTechnique { parents, members }
    }

    /// **O(log(n))**, marge a's trees and b's tree, if already united return false
    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let a_root = self.find(a);
        let b_root = self.find(b);
        if a_root != b_root {
            let (small_root, large_root) = (
                std::cmp::min_by_key(a_root, b_root, |&r| self.size(r)),
                std::cmp::max_by_key(a_root, b_root, |&r| self.size(r)),
            );
            self.parents[small_root] = large_root;
            let small_members = self.members.remove(&small_root).expect("above added");
            let large_members = self.members.get_mut(&large_root).expect("above added");
            large_members.extend(small_members);
        }
        a_root != b_root
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

    /// **O(log(n))**, return members in same group (warning: returned set size can be O(n))
    pub fn same_group(&mut self, x: usize) -> &HashSet<usize> {
        let root = self.find(x);
        &self.members[&root]
    }

    /// **O(log(n))**, return number of members in same group
    pub fn size(&mut self, x: usize) -> usize {
        self.same_group(x).len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_union() {
        let mut forest = MergeTechnique::new(5);
        forest.union(1, 2);
        forest.union(2, 3);
        assert_eq!(forest.size(0), 1);
        assert_eq!(forest.size(1), 3);
    }

    #[test]
    fn test_union2() {
        let mut forest = MergeTechnique::new(5);
        forest.union(2, 3);
        forest.union(1, 2);
        assert_eq!((0..5).map(|i| forest.find(i)).collect::<Vec<_>>(), vec![0, 3, 3, 3, 4]);
        assert_eq!(forest.union(1, 3), false);
    }

    #[test]
    fn test_find() {
        let mut forest = MergeTechnique::new(5);
        forest.union(0, 2);
        forest.union(2, 3);
        assert_eq!(forest.find(3), forest.find(0));
    }

    #[test]
    fn test_group() {
        let mut forest = MergeTechnique::new(10);
        for c in (0..10).collect::<Vec<_>>().chunks(2) {
            forest.union(c[0], c[1]);
        }
        forest.union(0, 2);
        forest.union(4, 6);
        forest.union(0, 4);
        assert_eq!(
            forest.same_group(0),
            &vec![0, 1, 2, 3, 4, 5, 6, 7].into_iter().collect::<HashSet<_>>()
        );
        assert_eq!(forest.same_group(9), &vec![8, 9].into_iter().collect::<HashSet<_>>());
        assert_eq!(forest.size(3), 8);
        assert_eq!(forest.size(8), 2);
    }
}
