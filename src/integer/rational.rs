use num::{integer::gcd, Integer};

#[derive(Clone, Copy, Eq, Ord, Debug)]
pub struct Rational<T: Integer + Copy> {
    // TODO: Rational<T> should be add/sub/mul/div T and &T and &Rational<T>
    // TODO: minus operator should be associated only with numerator
    numerator: T,
    denominator: T,
}
impl<T: Integer + Copy> Rational<T> {
    pub fn new(numerator: T, denominator: T) -> Option<Self> {
        if denominator == T::zero() {
            None
        } else {
            Some(Self { numerator, denominator })
        }
    }
    pub fn irreducible(&self) -> Self {
        let gcd = gcd(self.numerator, self.denominator);
        Self::new(self.numerator / gcd, self.denominator / gcd)
            .expect("denominator shouldn't be zero")
    }
    pub fn common(&self, other: &Self) -> (Self, Self) {
        let gcd = gcd(self.numerator, self.denominator);
        let denominator = self.denominator / gcd * other.denominator;
        let (s_numerator, o_numerator) = (
            denominator / self.denominator * self.numerator,
            denominator / other.denominator * other.numerator,
        );
        (
            Self::new(s_numerator, denominator).expect("denominator shouldn't be zero"),
            Self::new(o_numerator, denominator).expect("denominator shouldn't be zero"),
        )
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
        Self::new(numerator, denominator).expect("denominator shouldn't be zero").irreducible()
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
        Self::new(numerator, denominator).expect("denominator shouldn't be zero").irreducible()
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
        Self::new(numerator, denominator).expect("denominator shouldn't be zero").irreducible()
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
        Self::new(numerator, denominator).expect("denominator shouldn't be zero").irreducible()
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
    fn test_new() {
        assert_eq!(Rational::new(12, 36), Some(Rational { numerator: 12, denominator: 36 }));
        assert_eq!(Rational::new(12, 0), None);
    }

    #[test]
    fn test_eq() {
        let a = Rational::new(120, 70).expect("denominator shouldn't be zero");
        let b = Rational::new(48, 28).expect("denominator shouldn't be zero");
        assert_eq!(a, b);
    }

    #[test]
    fn test_ord() {
        let a = Rational::new(8, 9).expect("denominator shouldn't be zero");
        let b = Rational::new(10, 11).expect("denominator shouldn't be zero");
        assert!(a < b);
        assert!(a <= b);
    }

    #[test]
    fn test_add() {
        let a = Rational::new(5, 8).expect("denominator shouldn't be zero");
        let b = Rational::new(1, 6).expect("denominator shouldn't be zero");
        assert_eq!(a + b, Rational::new(19, 24).expect("denominator shouldn't be zero"));
    }

    #[test]
    fn test_add_assign() {
        let mut a = Rational::new(3, 4).expect("denominator shouldn't be zero");
        let b = Rational::new(1, 6).expect("denominator shouldn't be zero");
        a += b;
        assert_eq!(a, Rational::new(11, 12).expect("denominator shouldn't be zero"));
    }

    #[test]
    fn test_sub() {
        let a = Rational::new(5, 8).expect("denominator shouldn't be zero");
        let b = Rational::new(1, 6).expect("denominator shouldn't be zero");
        assert_eq!(a - b, Rational::new(11, 24).expect("denominator shouldn't be zero"));
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Rational::new(3, 4).expect("denominator shouldn't be zero");
        let b = Rational::new(1, 6).expect("denominator shouldn't be zero");
        a -= b;
        assert_eq!(a, Rational::new(7, 12).expect("denominator shouldn't be zero"));
    }

    #[test]
    fn test_mul() {
        let a = Rational::new(5, 8).expect("denominator shouldn't be zero");
        let b = Rational::new(1, 6).expect("denominator shouldn't be zero");
        assert_eq!(a * b, Rational::new(5, 48).expect("denominator shouldn't be zero"));
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Rational::new(3, 4).expect("denominator shouldn't be zero");
        let b = Rational::new(1, 6).expect("denominator shouldn't be zero");
        a *= b;
        assert_eq!(a, Rational::new(1, 8).expect("denominator shouldn't be zero"));
    }

    #[test]
    fn test_div() {
        let a = Rational::new(5, 8).expect("denominator shouldn't be zero");
        let b = Rational::new(1, 6).expect("denominator shouldn't be zero");
        assert_eq!(a / b, Rational::new(15, 4).expect("denominator shouldn't be zero"));
    }

    #[test]
    fn test_div_assign() {
        let mut a = Rational::new(3, 4).expect("denominator shouldn't be zero");
        let b = Rational::new(1, 6).expect("denominator shouldn't be zero");
        a /= b;
        assert_eq!(a, Rational::new(9, 2).expect("denominator shouldn't be zero"));
    }

    #[test]
    #[should_panic]
    fn test_div_by_zero() {
        let a = Rational::new(12, 5).expect("denominator shouldn't be zero");
        let b = Rational::new(0, 1).expect("denominator shouldn't be zero");
        let _ = a / b;
    }
}
