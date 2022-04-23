/// **O(n)**, get iterator of two pointer method. (if cond is true progress a, else b)
pub fn two_pointers<T, Ia, Ib, F>(a: Ia, b: Ib, cond: F) -> std::vec::IntoIter<(T, T)>
where
    T: Clone,
    Ia: IntoIterator<Item = T>,
    Ib: IntoIterator<Item = T>,
    F: Fn(&T, &T) -> bool,
{
    // TODO: do not use Vec
    let b: Vec<_> = b.into_iter().collect();
    let (mut j, mut two_pointers) = (0, Vec::new());
    for ai in a.into_iter() {
        two_pointers.push((ai.clone(), b[j].clone()));
        while j < b.len() - 1 && !cond(&ai, &b[j]) {
            j = b.len().min(j + 2) - 1;
            two_pointers.push((ai.clone(), b[j].clone()));
        }
    }
    two_pointers.into_iter()
}

#[cfg(test)]
mod tests {
    use num::Signed;

    use super::*;
    #[test]
    fn test_min_delta() {
        let a = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let b = vec![2, 4, 8, 16, 32, 64, 128];
        {
            let mut min = 100;
            for (ai, bj) in two_pointers(a.iter(), b.iter(), |ai, bj| ai < bj) {
                min = min.min((ai - bj).abs());
            }
            assert_eq!(min, 2);
        }
        {
            let mut min = 100;
            for (bj, ai) in two_pointers(b, a, |bj, ai| bj < ai) {
                min = min.min((ai - bj).abs());
            }
            assert_eq!(min, 2);
        }
    }
}
