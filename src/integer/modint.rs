use num::Integer;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ModInt<T: Integer + Copy> {
    // TODO: to represent modulo, use type semantics
    int: T,
    modulo: T,
}
impl<T: Integer + Copy> ModInt<T> {
    pub fn new(int: T, modulo: T) -> Self {
        Self { int: int % modulo, modulo }
    }
}
impl<T: std::fmt::Display + Integer + Copy> std::fmt::Display for ModInt<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.int % self.modulo)
    }
}
impl<T: Integer + Copy> std::ops::Add for ModInt<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let result = (self.int + rhs.int) % self.modulo;
        Self::new(result, self.modulo)
    }
}
impl<T: Integer + Copy> std::ops::AddAssign for ModInt<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl<T: Integer + Copy> std::ops::Sub for ModInt<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let result = (self.int + self.modulo - rhs.int) % self.modulo;
        Self::new(result, self.modulo)
    }
}
impl<T: Integer + Copy> std::ops::SubAssign for ModInt<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl<T: Integer + Copy> std::ops::Mul for ModInt<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let result = (self.int * rhs.int) % self.modulo;
        Self::new(result, self.modulo)
    }
}
impl<T: Integer + Copy> std::ops::MulAssign for ModInt<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add() {
        let a = ModInt::new(5, 8);
        let b = ModInt::new(7, 8);
        assert_eq!(a + b, ModInt::new(4, 8));
    }

    #[test]
    fn test_add_assign() {
        let mut a = ModInt::new(1, 19);
        let b = ModInt::new(3, 19);
        a += b;
        assert_eq!(a, ModInt::new(4, 19));
    }

    #[test]
    fn test_sub() {
        let a = ModInt::new(1, 6);
        let b = ModInt::new(5, 6);
        assert_eq!(a - b, ModInt::new(2, 6));
    }

    #[test]
    fn test_sub_assign() {
        let mut a = ModInt::new(3, 4);
        let b = ModInt::new(1, 4);
        a -= b;
        assert_eq!(a, ModInt::new(2, 4));
    }

    #[test]
    fn test_mul() {
        let a = ModInt::new(2, 5);
        let b = ModInt::new(7, 5);
        assert_eq!(a * b, ModInt::new(4, 5));
    }

    #[test]
    fn test_mul_assign() {
        let mut a = ModInt::new(12, 59);
        let b = ModInt::new(15, 59);
        a *= b;
        assert_eq!(a, ModInt::new(3, 59));
    }
}
