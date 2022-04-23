#[macro_export]
/// **O(n)**, max for one or more values
macro_rules! max {
    ( $first:expr ) => { $first };

    ( $first:expr, $($args:expr),* ) => {
        {
            let mut result = $first;
            $(
                result = std::cmp::max(result, $args);
            )*
            result
        }
    };
}

#[macro_export]
/// **O(n)**, min for one or more values
macro_rules! min {
    ( $first:expr ) => { $first };

    ( $first:expr, $($args:expr),* ) => {
        {
            let mut result = $first;
            $(
                result = std::cmp::min(result, $args);
            )*
            result
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_max() {
        assert_eq!(max!(1, 5, 100, 123, 4312, 2, 12, 412), 4312);
        assert_eq!(max!(10, 12, 12, 12), 12);
        assert_eq!(max!(0, 2), 2);
        assert_eq!(max!(-100), -100);
    }

    #[test]
    fn test_min() {
        assert_eq!(min!(1, 5, 100, -123, 4312, -2, 12, 412), -123);
        assert_eq!(min!(10, 10, 10, 12), 10);
        assert_eq!(min!(0, 2), 0);
        assert_eq!(min!(-100), -100);
    }
}
