#[allow(dead_code)]
use super::{
    dp::{knapsack_dp_value_solve, knapsack_dp_weight},
    meet_in_the_middle::knapsack_half_enumerate,
};

use num::{cast::AsPrimitive, Integer, NumCast};
use std::hash::Hash;

pub fn knapsack<V, W>(n: usize, c: W, w: &[W], v: &[V]) -> V
where
    W: Integer + AsPrimitive<usize> + Into<usize> + Copy + Hash,
    V: Integer + NumCast + AsPrimitive<usize> + Copy,
{
    if n <= 40 {
        knapsack_half_enumerate(n, c, w, v)
    } else if !n.overflowing_mul(c.as_()).1 {
        let c = c.as_();
        let w: Vec<_> = w.iter().map(|wi| wi.as_()).collect();
        knapsack_dp_weight(n, c, &w, &v)[n][c]
    } else {
        let v: Vec<_> = v.iter().map(|vi| vi.as_()).collect();
        V::from(knapsack_dp_value_solve(n, c, &w, &v)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    // test cases from https://atcoder.jp/contests/abc032/tasks/abc032_d
    use super::*;

    #[test]
    fn knapsack_test1() {
        let (n, c) = (3, 10usize);
        let (w, v) = ([9, 6, 4], [15, 10, 6]);
        assert_eq!(knapsack(n, c, &w, &v), 16);
    }

    #[test]
    fn knapsack_test2() {
        let (n, c) = (30, 499887702usize);
        let w = [
            137274936, 989051853, 85168425, 856699603, 611065509, 22345022, 678298936, 616908153,
            28801762, 478675378, 706900574, 738510039, 135746508, 599020879, 738084616, 545330137,
            86797589, 592749599, 401229830, 523386474, 5310725, 907821957, 565237085, 730556272,
            310581512, 136966252, 132739489, 12425915, 137199296, 23505143,
        ];
        let v = [
            128990795, 575374246, 471048785, 640066776, 819841327, 704171581, 536108301, 119980848,
            117241527, 325850062, 623319578, 998395208, 475707585, 863910036, 340559411, 122579234,
            696368935, 665665204, 958833732, 371084424, 463433600, 210508742, 685281136, 619500108,
            88215377, 558193168, 475268130, 303022740, 122379996, 304092766,
        ];
        assert_eq!(knapsack(n, c, &w, &v), 3673016420u128);
    }

    #[test]
    fn knapsack_test3() {
        let (n, c) = (10, 2921usize);
        let w = [325, 845, 371, 112, 96, 960, 161, 581, 248, 22];
        let v = [
            981421680, 515936168, 17309336, 788067075, 104855562, 494541604, 32007355, 772339969,
            55112800, 98577050,
        ];
        assert_eq!(knapsack(n, c, &w, &v), 3657162058usize);
    }

    #[test]
    fn knapsack_test4() {
        let (n, c) = (10, 936447862usize);
        let w = [
            810169801, 957981784, 687140254, 932608409, 42367415, 727293784, 870916042, 685539955,
            243593312, 977358410,
        ];
        let v = [854, 691, 294, 333, 832, 642, 139, 101, 853, 369];
        assert_eq!(knapsack(n, c, &w, &v), 1686usize);
    }
}
