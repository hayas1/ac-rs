use std::iter::successors;

/// **O(n log(n))**, calculate inversion number, on data such as permutation of 0,1,2,...,n-1
pub fn inversion_number(data: &[usize]) -> usize {
    let mut bit = vec![0; data.len()];
    data.iter().enumerate().fold(0, |sum, (i, &di)| {
        let prefix_sums = successors(Some(di), |&j| {
            Some(j - if j == 0 { 0 } else { 2usize.pow(j.trailing_zeros()) })
        })
        .take_while(|&j| 0 < j)
        .fold(0, |p_sum, j| p_sum + bit[j]);
        successors(Some(di), |&j| {
            Some(j + 2usize.pow(if j == 0 { 0 } else { j.trailing_zeros() }))
        })
        .take_while(|&j| j < data.len())
        .for_each(|j| bit[j] += 1);
        sum + i - prefix_sums
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn inversion_number_test1() {
        assert_eq!(inversion_number(&[0, 1, 2, 3]), 0);
        assert_eq!(inversion_number(&[3, 2, 1, 0]), 6);
        assert_eq!(inversion_number(&[0, 3, 1, 5, 4, 2, 9, 6, 8, 7]), 9);
        assert_eq!(inversion_number(&[4, 3, 0, 2, 1, 5]), 8);
    }
}
