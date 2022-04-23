#![allow(unused_mut)]

#[macro_export]
/// **O(n)**, sum for values
macro_rules! sum {

    ( $($args:expr),* ) => {
        {
            let mut result = 0;
            $(
                result += $args;
            )*
            result
        }
    };
}

#[macro_export]
/// **O(n)**, prod for values
macro_rules! prod {

    ( $($args:expr),* ) => {
        {
            let mut result = 1;
            $(
                result *= $args;
            )*
            result
        }
    };
}

#[macro_export]
/// **O(n)**, average for values
macro_rules! mean {

    ( $($args:expr),* ) => {
        {
            let (mut sum, mut num) = (0, 0);
            $(
                sum += $args;
                num += 1;
            )*
            if num == 0 {
                sum as f64
            } else {
                sum as f64 / num as f64
            }
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_sum() {
        assert_eq!(sum!(1, 5, 100, 123, 4312, 2, 12, 412), 4967);
        assert_eq!(sum!(10, 12, 12, 12), 46);
        assert_eq!(sum!(0, 2), 2);
        assert_eq!(sum!(-100), -100);
        assert_eq!(sum!(), 0);
    }

    #[test]
    fn test_prod() {
        assert_eq!(prod!(1u64, 5, 100, 123, 4312, 2, 12, 412), 2622178944000);
        assert_eq!(prod!(10, 12, 12, 12), 17280);
        assert_eq!(prod!(0, 2), 0);
        assert_eq!(prod!(-100), -100);
        assert_eq!(prod!(), 1);
    }

    #[test]
    fn test_mean() {
        assert_eq!(mean!(1u64, 5, 100, 123, 4312, 2, 12, 412), 620.875);
        assert_eq!(mean!(10, 12, 12, 12), 11.5);
        assert_eq!(mean!(0, 2), 1.);
        assert_eq!(mean!(-100), -100.);
        assert_eq!(mean!(), 0.);
    }
}
