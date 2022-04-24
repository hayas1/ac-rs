use num::{integer::gcd, Integer};

#[derive(Clone, Copy, Eq, Ord, Debug)]
pub struct Rational<T: Integer + Copy> {
    // TODO: Rational<T> should be add/sub/mul/div T and &T and &Rational<T>
    // TODO: if denominator is 0, rational should not be made
    // TODO: minus operator should be associated only with numerator
    numerator: T,
    denominator: T,
}
impl<T: Integer + Copy> Rational<T> {
    pub fn irreducible(&self) -> Self {
        let gcd = gcd(self.numerator, self.denominator);
        Self { numerator: self.numerator / gcd, denominator: self.denominator / gcd }
    }
    pub fn common(&self, other: &Self) -> (Self, Self) {
        let gcd = gcd(self.numerator, self.denominator);
        let denominator = self.denominator / gcd * other.denominator;
        let (sn, on) = (
            denominator / self.denominator * self.numerator,
            denominator / other.denominator * other.numerator,
        );
        (Self { numerator: sn, denominator }, Self { numerator: on, denominator })
    }
}
impl<T: std::fmt::Display + Integer + Copy> std::fmt::Display for Rational<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}
impl<T: Integer + Copy> PartialEq for Rational<T> {
    fn eq(&self, other: &Self) -> bool {
        let (si, oi) = (self.irreducible(), other.irreducible());
        si.numerator == oi.numerator && si.denominator == oi.denominator
    }
}
impl<T: Integer + Copy> PartialOrd for Rational<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (sc, oc) = self.common(other);
        sc.numerator.partial_cmp(&oc.numerator)
    }
}
impl<T: Integer + Copy> std::ops::Add for Rational<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (sc, rc) = self.common(&rhs);
        let (numerator, denominator) = (sc.numerator + rc.numerator, sc.denominator);
        Self { numerator, denominator }.irreducible()
    }
}
impl<T: Integer + Copy> std::ops::AddAssign for Rational<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl<T: Integer + Copy> std::ops::Sub for Rational<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let (sc, rc) = self.common(&rhs);
        let (numerator, denominator) = (sc.numerator - rc.numerator, sc.denominator);
        Self { numerator, denominator }.irreducible()
    }
}
impl<T: Integer + Copy> std::ops::SubAssign for Rational<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl<T: Integer + Copy> std::ops::Mul for Rational<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let (sc, rc) = self.common(&rhs);
        let (numerator, denominator) =
            (sc.numerator * rc.numerator, sc.denominator * rc.denominator);
        Self { numerator, denominator }.irreducible()
    }
}
impl<T: Integer + Copy> std::ops::MulAssign for Rational<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl<T: Integer + Copy> std::ops::Div for Rational<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let (sc, rc) = self.common(&rhs);
        let (numerator, denominator) =
            (sc.numerator * rc.denominator, sc.denominator * rc.numerator);
        Self { numerator, denominator }.irreducible()
    }
}
impl<T: Integer + Copy> std::ops::DivAssign for Rational<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_eq() {
        let a = Rational { numerator: 120, denominator: 70 };
        let b = Rational { numerator: 48, denominator: 28 };
        assert_eq!(a, b);
    }

    #[test]
    fn test_ord() {
        let a = Rational { numerator: 8, denominator: 9 };
        let b = Rational { numerator: 10, denominator: 11 };
        assert!(a < b);
        assert!(a <= b);
    }

    #[test]
    fn test_add() {
        let a = Rational { numerator: 5, denominator: 8 };
        let b = Rational { numerator: 1, denominator: 6 };
        assert_eq!(a + b, Rational { numerator: 19, denominator: 24 });
    }

    #[test]
    fn test_add_assign() {
        let mut a = Rational { numerator: 3, denominator: 4 };
        let b = Rational { numerator: 1, denominator: 6 };
        a += b;
        assert_eq!(a, Rational { numerator: 11, denominator: 12 });
    }

    #[test]
    fn test_sub() {
        let a = Rational { numerator: 5, denominator: 8 };
        let b = Rational { numerator: 1, denominator: 6 };
        assert_eq!(a - b, Rational { numerator: 11, denominator: 24 });
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Rational { numerator: 3, denominator: 4 };
        let b = Rational { numerator: 1, denominator: 6 };
        a -= b;
        assert_eq!(a, Rational { numerator: 7, denominator: 12 });
    }

    #[test]
    fn test_mul() {
        let a = Rational { numerator: 5, denominator: 8 };
        let b = Rational { numerator: 1, denominator: 6 };
        assert_eq!(a * b, Rational { numerator: 5, denominator: 48 });
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Rational { numerator: 3, denominator: 4 };
        let b = Rational { numerator: 1, denominator: 6 };
        a *= b;
        assert_eq!(a, Rational { numerator: 1, denominator: 8 });
    }

    #[test]
    fn test_div() {
        let a = Rational { numerator: 5, denominator: 8 };
        let b = Rational { numerator: 1, denominator: 6 };
        assert_eq!(a / b, Rational { numerator: 15, denominator: 4 });
    }

    #[test]
    fn test_div_assign() {
        let mut a = Rational { numerator: 3, denominator: 4 };
        let b = Rational { numerator: 1, denominator: 6 };
        a /= b;
        assert_eq!(a, Rational { numerator: 9, denominator: 2 });
    }
}
