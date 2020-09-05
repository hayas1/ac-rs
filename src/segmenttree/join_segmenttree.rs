#![allow(dead_code)]

struct JoinSegmentTree {
    n: usize,
    binary_tree: Vec<String>,
}
impl JoinSegmentTree {
    /// O(n) # create segment tree (its monoid function is add, so we can calculate summation)
    fn new(data: &[String]) -> Self {
        let n = data.len();
        let mut binary_tree = vec!["".to_string(); 2 * n.next_power_of_two() - 1];
        for (i, di) in data.iter().enumerate() {
            binary_tree[n.next_power_of_two() - 1 + i] = di.clone();
        }
        for i in (0..n.next_power_of_two() - 1).rev() {
            binary_tree[i] = format!("{}{}", &binary_tree[2 * i + 1], &binary_tree[2 * i + 2]);
        }
        JoinSegmentTree { n, binary_tree }
    }

    /// O(log(n)) # update segment tree, leaf[k] = x
    fn update(&mut self, k: usize, x: &str) {
        let i = self.n.next_power_of_two() - 1 + k;
        self.binary_tree[i] = x.to_string();
        if i > 0 {
            self.recursive_update((i - 1) / 2);
        }
    }

    /// O(log(i)) # update from leaf to root
    fn recursive_update(&mut self, i: usize) {
        self.binary_tree[i] = format!(
            "{}{}",
            &self.binary_tree[2 * i + 1],
            &self.binary_tree[2 * i + 2]
        );
        if i > 0 {
            self.recursive_update((i - 1) / 2);
        }
    }

    /// O(log(n)) # calculate half-open interval summation from l to r (leaf index)
    fn query(&self, l: usize, r: usize) -> String {
        self.recursive_query(l, r, 0, 0, self.n.next_power_of_two())
    }

    /// O(log(n)-log(node)) # calculate summation from root to leaf
    fn recursive_query(&self, l: usize, r: usize, node: usize, from: usize, to: usize) -> String {
        if r <= from || to <= l {
            "".to_string()
        } else if l <= from && to <= r {
            self.binary_tree[node].clone()
        } else {
            format!(
                "{}{}",
                &self.recursive_query(l, r, node * 2 + 1, from, (from + to) / 2),
                &self.recursive_query(l, r, node * 2 + 2, (from + to) / 2, to)
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_test() {
        let data: Vec<_> = ["r", "u", "s", "t", "a", "b", "c"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut t = JoinSegmentTree::new(&data);
        assert_eq!(
            t.binary_tree,
            vec![
                "rustabc", "rust", "abc", "ru", "st", "ab", "c", "r", "u", "s", "t", "a", "b", "c",
                ""
            ]
        );
        t.update(2, "b");
        t.update(3, "y");
        assert_eq!(
            t.binary_tree,
            vec![
                "rubyabc", "ruby", "abc", "ru", "by", "ab", "c", "r", "u", "b", "y", "a", "b", "c",
                ""
            ]
        );
    }

    #[test]
    fn query_test() {
        let data: Vec<_> = ["r", "u", "s", "t", "a", "b", "c"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let t = JoinSegmentTree::new(&data);
        assert_eq!(t.query(0, 4), "rust");
        assert_eq!(t.query(4, 7), "abc");
        assert_eq!(t.query(1, 3), "us");
        assert_eq!(t.query(0, 0), "");
    }

    #[test]
    fn update_query_test() {
        let data: Vec<_> = ["r", "u", "s", "t", "a", "b", "c"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut t = JoinSegmentTree::new(&data);
        assert_eq!(t.query(0, 4), "rust");
        assert_eq!(t.query(4, 7), "abc");
        assert_eq!(t.query(1, 3), "us");
        assert_eq!(t.query(0, 0), "");
        t.update(2, "b");
        t.update(3, "y");
        assert_eq!(t.query(0, 4), "ruby");
        assert_eq!(t.query(4, 7), "abc");
        assert_eq!(t.query(1, 3), "ub");
        assert_eq!(t.query(0, 0), "");
    }
}
