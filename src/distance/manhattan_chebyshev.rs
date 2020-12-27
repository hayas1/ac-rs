#![allow(dead_code)]

use num::Integer;

/// **O(1)** calculate manhattan distance between two point of 2-dimension
pub fn manhattan_distance<T: Integer + Copy>(p1: (T, T), p2: (T, T)) -> T {
    let ((x1, y1), (x2, y2)) = (p1, p2);
    (x1.max(x2) - x1.min(x2)) + (y1.max(y2) - y1.min(y2))
}

/// **O(n)** 45-degree rotation, manhattan distance become to be chebyshev distance
pub fn rotate_45<T: Integer + Copy>(p: &[(T, T)]) -> Vec<(T, T)> {
    p.iter().map(|&(x, y)| (x - y, x + y)).collect()
}

/// **O(1)** calculate chebyshev distance between two point of 2-dimension
pub fn chebyshev_distance<T: Integer + Copy>(p1: (T, T), p2: (T, T)) -> T {
    let ((x1, y1), (x2, y2)) = (p1, p2);
    let (max_x, max_y) = (x1.max(x2) - x1.min(x2), y1.max(y2) - y1.min(y2));
    max_x.max(max_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manhattan_test() {
        assert_eq!(manhattan_distance((0, 0), (-1, 2)), 3);
        assert_eq!(manhattan_distance((1, 2), (-1, -2)), 6);
        assert_eq!(manhattan_distance((10, 10), (10, 10)), 0);
    }

    #[test]
    fn rotate45_test() {
        let p = [(0, 0), (-1, 2), (1, 2), (-1, -2), (10, 10)];
        assert_eq!(rotate_45(&p), [(0, 0), (-3, 1), (-1, 3), (1, -3), (0, 20)]);
    }

    #[test]
    fn chebyshev_test() {
        assert_eq!(chebyshev_distance((0, 0), (-3, 1)), 3);
        assert_eq!(chebyshev_distance((-1, 3), (1, -3)), 6);
        assert_eq!(chebyshev_distance((0, 20), (0, 20)), 0);
    }

    #[test]
    fn manhattan_to_chebyshev_test() {
        let p = [(0, 0), (-1, 2), (1, 2), (-1, -2), (10, 10)];
        let rotated = rotate_45(&p);
        for (&pp, &rp) in p.iter().zip(rotated.iter()) {
            assert_eq!(
                manhattan_distance((0, 0), pp),
                chebyshev_distance((0, 0), rp)
            );
        }
        for i in 0..p.len() {
            for j in 0..p.len() {
                assert_eq!(
                    manhattan_distance(p[i], p[j]),
                    chebyshev_distance(rotated[i], rotated[j])
                );
            }
        }
    }
}
