use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct TrieTree<E> {
    root: TrieNode<E>,
    len: usize,
}
#[derive(Debug)]
struct TrieNode<E> {
    children: HashMap<E, Self>,
}

impl<E> TrieTree<E>
where
    E: Hash + Eq + Clone,
{
    /// **O(1)**, make empty trie tree
    pub fn new() -> Self {
        let root = TrieNode { children: HashMap::new() };
        TrieTree { root, len: 0 }
    }

    /// **O(m)**, search prefix and return node and remain keys
    fn search<K>(&self, key: K) -> (&TrieNode<E>, Option<E>, K::IntoIter)
    where
        K: IntoIterator<Item = E>,
    {
        let mut current = &self.root;
        let mut iter = key.into_iter();
        while let Some(k) = iter.next() {
            if let Some(next) = current.children.get(&k) {
                current = next;
            } else {
                return (current, Some(k), iter);
            }
        }
        (current, None, iter)
    }

    /// **O(m)**, search prefix and return mutable node and remain keys
    fn search_mut<K>(&mut self, key: K) -> (&mut TrieNode<E>, Option<E>, K::IntoIter)
    where
        K: IntoIterator<Item = E>,
    {
        // TODO: without raw pointer and unsafe ... ?
        let mut current = &mut self.root as *mut TrieNode<E>;
        let mut iter = key.into_iter();
        while let Some(k) = iter.next() {
            if let Some(next) = unsafe { &mut *current }.children.get_mut(&k) {
                current = next;
            } else {
                return (unsafe { &mut *current }, Some(k), iter);
            }
        }
        (unsafe { &mut *current }, None, iter)
    }

    /// **O(m)**, insert key
    pub fn insert<K>(&mut self, key: K) -> bool
    where
        K: IntoIterator<Item = E>,
    {
        let (mut current, head, remain) = self.search_mut(key);
        match head {
            Some(h) => {
                for key_element in vec![h].into_iter().chain(remain) {
                    current = current
                        .children
                        .entry(key_element)
                        .or_insert_with(|| TrieNode { children: HashMap::new() })
                }
                self.len += 1;
                true
            }
            None => false,
        }
    }

    /// **O(m)**, remove key (WARNING: unimplemented!)
    pub fn remove<K>(&mut self, _key: K) -> bool
    where
        K: IntoIterator<Item = E>,
    {
        todo!()
    }

    /// **O(m)**, judge key is inserted or not
    pub fn contains<K>(&self, key: K) -> bool
    where
        K: IntoIterator<Item = E>,
    {
        let (current, head, _) = self.search(key);
        current.children.is_empty() && head.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert_contains() {
        let mut trie = TrieTree::<char>::new();
        assert!(!trie.contains("rust".chars()));
        trie.insert("rust".chars());
        assert!(trie.contains("rust".chars()));
        assert!(!trie.contains("rus".chars()));
        assert!(!trie.contains("ust".chars()));
        trie.insert("ruby".chars());
        assert!(trie.contains("rust".chars()));
        assert!(trie.contains("ruby".chars()));
        assert!(!trie.contains("ru".chars()));
        println!("{:?}", trie);
    }
}
