#![allow(dead_code)]

use num::Integer;

/// O(sum(v)n) # knapsack capacity is c, value of pi is v[i], weight of p_i is w[i]
pub fn knapsack_dp_value<W: Integer + Copy>(
    n: usize,
    _c: W,
    w: &[W],
    v: &[usize],
) -> Vec<Vec<Option<W>>> {
    let sum_v = v.iter().fold(0, |sum, x| sum + x);
    // dp[i][j] = min of sum of weight, such as sum of value is j and product[0..=i]
    let mut dp = vec![vec![None; sum_v + 1]; n]; // initiate by infinity
    dp[0][0] = Some(W::zero());
    dp[0][v[0]] = Some(w[0]);
    for i in 1..n {
        dp[i][0] = Some(W::zero());
        for j in 1..=sum_v {
            if v[i] <= j {
                dp[i][j] = match dp[i - 1][j - v[i]] {
                    Some(p) => match dp[i - 1][j] {
                        Some(l) => Some(l.min(w[i] + p)),
                        None => Some(w[i] + p),
                    },
                    None => dp[i - 1][j],
                };
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }
    dp
}

pub fn knapsack_dp_value_solve<W: Integer + Copy>(n: usize, c: W, w: &[W], v: &[usize]) -> usize {
    let dp_table = knapsack_dp_value(n, c, &w, &v);
    let mut maxarg = 0;
    for j in 0..v.iter().fold(0, |sum, x| sum + x) {
        match dp_table[n - 1][j] {
            Some(x) => {
                if x <= c {
                    maxarg = j;
                }
            }
            None => continue,
        }
    }
    maxarg
}

/// O(cn) # knapsack capacity is c, value of pi is v[i], weight of p_i is w[i]
pub fn knapsack_dp_weight<V: Integer + Copy>(
    n: usize,
    c: usize,
    w: &[usize],
    v: &[V],
) -> Vec<Vec<V>> {
    // dp[i][j] = max of sum of value, such as sum of weight is at most j and product[0..=i]
    let mut dp = vec![vec![V::zero(); c + 1]; n + 1];
    for i in 0..n {
        dp[i][0] = V::zero();
        for j in 0..=c {
            dp[i + 1][j] = if j >= w[i] {
                // if we can update the value by taking p_i, do so
                dp[i][j].max(dp[i][j - w[i]] + v[i])
            } else {
                // otherwise, we do not take it
                dp[i][j]
            };
        }
    }
    dp
}

/// O(cn) # compute vec of products from table made by knapsack_dp_weight()
pub fn dp_weight_with_backtrack<V: Integer + Copy>(
    n: usize,
    c: usize,
    w: &[usize],
    v: &[V],
) -> Vec<usize> {
    let dp = knapsack_dp_weight(n, c, w, v);
    let mut taken = Vec::new();
    let (mut value, mut weight) = (dp[v.len()][c], c);
    for i in (0..v.len()).rev() {
        if value <= V::zero() {
            break;
        } else if value == dp[i][weight] {
            continue;
        } else {
            taken.push(i);
            value = value - v[i];
            weight = weight - w[i];
        }
    }
    return taken;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn knapsack_dp_value_test1() {
        let (n, c) = (4, 10);
        let (w, v) = (vec![4, 7, 2, 4], vec![1, 3, 1, 2]);

        let dp_result = vec![
            vec![Some(0), Some(4), None, None, None, None, None, None],
            vec![Some(0), Some(4), None, Some(7), Some(11), None, None, None],
            vec![
                Some(0),
                Some(2),
                Some(6),
                Some(7),
                Some(9),
                Some(13),
                None,
                None,
            ],
            vec![
                Some(0),
                Some(2),
                Some(4),
                Some(6),
                Some(9), // <- max j such as result[n-1][j] less than c
                Some(11),
                Some(13),
                Some(17),
            ],
        ];
        assert_eq!(knapsack_dp_value(n, c, &w, &v), dp_result);
        assert_eq!(knapsack_dp_value_solve(n, c, &w, &v), 4);
    }

    #[test]
    fn knapsack_dp_value_test2() {
        let (n, c) = (10, 936447862usize);
        let w = [
            810169801, 957981784, 687140254, 932608409, 42367415, 727293784, 870916042, 685539955,
            243593312, 977358410,
        ];
        let v = [854, 691, 294, 333, 832, 642, 139, 101, 853, 369];
        assert_eq!(knapsack_dp_value_solve(n, c, &w, &v), 1686);
    }

    #[test]
    fn knapsack_dp_weight_test1() {
        let (n, c) = (4, 10);
        let (w, v) = (vec![4, 7, 2, 4], vec![1, 3, 1, 2]);
        let dp_result = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 1, 1, 1, 3, 3, 3, 3],
            [0, 0, 1, 1, 1, 1, 2, 3, 3, 4, 4],
            [0, 0, 1, 1, 2, 2, 3, 3, 3, 4, 4],
        ];
        assert_eq!(knapsack_dp_weight(n, c, &w, &v), dp_result);
    }

    #[test]
    fn knapsack_dp_weight_test2() {
        let (n, c) = (6, 15);
        let w = vec![2, 1, 3, 2, 1, 5];
        let v = vec![3, 2, 6, 1, 3, 85];
        let dp_result = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
            [0, 2, 3, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5],
            [0, 2, 3, 6, 8, 9, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11],
            [0, 2, 3, 6, 8, 9, 11, 11, 12, 12, 12, 12, 12, 12, 12, 12],
            [0, 3, 5, 6, 9, 11, 12, 14, 14, 15, 15, 15, 15, 15, 15, 15],
            [0, 3, 5, 6, 9, 85, 88, 90, 91, 94, 96, 97, 99, 99, 100, 100],
        ];
        assert_eq!(knapsack_dp_weight(n, c, &w, &v), dp_result);
    }

    #[test]
    fn knapsack_dp_weight_test3() {
        let (n, c) = (10, 2921);
        let w = [325, 845, 371, 112, 96, 960, 161, 581, 248, 22];
        let v = [
            981421680, 515936168, 17309336, 788067075, 104855562, 494541604, 32007355, 772339969,
            55112800, 98577050,
        ];
        assert_eq!(knapsack_dp_weight(n, c, &w, &v)[n][c], 3657162058usize);
    }

    #[test]
    fn knapsack_dp_weight_with_backtrack_test1() {
        let (n, c) = (6, 15);
        let w = vec![2, 1, 3, 2, 1, 5];
        let v = vec![3, 2, 6, 1, 3, 85];
        assert_eq!(dp_weight_with_backtrack(n, c, &w, &v), [5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn knapsack_dp_weight_with_backtrack_test2() {
        let (n, c) = (4, 15);
        let w = vec![10, 3, 5, 7];
        let v = vec![6, 7, 2, 4];
        // assert_eq!(dp_weight_with_backtrack(c, &w, &v), [3, 2, 1]);
        assert_eq!(dp_weight_with_backtrack(n, c, &w, &v), [1, 0]);
        assert_eq!(
            dp_weight_with_backtrack(n, c, &w, &v)
                .iter()
                .fold(0, |sum, &x| sum + v[x]),
            13
        );
    }

    #[test]
    fn knapsack_dp_weight_with_backtrack_test3() {
        let (n, c) = (5, 15);
        let w = [4, 2, 2, 1, 10];
        let v = [12, 2, 1, 1, 4];
        assert_eq!(dp_weight_with_backtrack(n, c, &w, &v), [4, 3, 0]);
    }

    #[test]
    fn knapsack_dp_weight_with_backtrack_test4() {
        let (n, c) = (3, 50);
        let w = [10, 20, 30];
        let v = [60, 100, 120];
        let result = dp_weight_with_backtrack(n, c, &w, &v);
        assert_eq!(result, [2, 1]);
        assert_eq!(result.iter().fold(0, |sum, &x| sum + v[x]), 220);
    }

    // #[test]
    // fn knapsack_dp_value_empty_test() {
    //     let (n, c) = (0, 10);
    //     let (w, v) = (Vec::new(), Vec::new());
    //     assert_eq!(knapsack_dp_value(n, c, &w, &v), vec![vec![None]]);
    // }

    #[test]
    fn knapsack_dp_weight_empty_test() {
        let (n, c) = (0, 10);
        let (w, v) = (Vec::new(), Vec::<usize>::new());
        assert_eq!(knapsack_dp_weight(n, c, &w, &v), vec![vec![0; 11]]);
    }

    #[test]
    fn knapsack_dp_weight_with_backtrack_empty_test() {
        let (n, c) = (0, 10);
        let (w, v) = (Vec::new(), Vec::<usize>::new());
        assert_eq!(dp_weight_with_backtrack(n, c, &w, &v), vec![]);
    }
}
