#![allow(unused_macros)]

#[macro_export]
/// **O(n)**, max for one or more values
macro_rules! max {
    ( $head:expr ) => { $head };

    ( $head:expr, $($tail:expr), + ) => {
        std::cmp::max($head, max!($($tail), +))
    };
}

#[macro_export]
/// **O(n)**, min for one or more values
macro_rules! min {
    ( $head:expr ) => { $head };

    ( $head:expr, $($tail:expr), + ) => {
        std::cmp::min($head, min!($($tail), +))
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn max_test() {
        assert_eq!(max!(1, 5, 100, 123, 4312, 2, 12, 412), 4312);
        assert_eq!(max!(10, 12, 12, 12), 12);
        assert_eq!(max!(0, 2), 2);
        assert_eq!(max!(-100), -100);
    }

    #[test]
    fn min_test() {
        assert_eq!(min!(1, 5, 100, -123, 4312, -2, 12, 412), -123);
        assert_eq!(min!(10, 10, 10, 12), 10);
        assert_eq!(min!(0, 2), 0);
        assert_eq!(min!(-100), -100);
    }
}
