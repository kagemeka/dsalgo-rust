use std::{
    ops::*,
    str::FromStr,
};

use crate::{
    greatest_common_divisor_euclidean_recurse_i64::gcd,
    least_common_multiple_with_gcd_i64::lcm,
    multiplicative_inverse::MulInv,
};

/// (numerator, denominator)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord)]

pub struct Fraction(pub i64, pub i64);

impl Fraction {
    /// lower != 0, but accept infinity ?

    pub fn new(
        upper: i64,
        lower: i64,
    ) -> Self {
        // assert!(lower != 0);
        assert!(upper != 0 || lower != 0);

        let mut v = Self(upper, lower);

        v.normalize();

        v
    }

    fn make_denominator_positive(&mut self) {
        if self.1 < 0 {
            self.0 = -self.0;

            self.1 = -self.1;
        }
    }

    fn reduction(&mut self) {
        let g = gcd(self.0, self.1);

        debug_assert!(g != 0);

        self.0 /= g;

        self.1 /= g;
    }

    fn normalize(&mut self) {
        self.reduction();

        self.make_denominator_positive();
    }

    pub fn normalize_neg_inf_as_inf(&mut self) {
        if self.0 == -1 && self.1 == 0 {
            self.0 = 1;
        }
    }

    pub fn floor(&self) -> i64 { self.0.div_euclid(self.1) }

    pub fn ceil(&self) -> i64 {
        let mut v = self.floor();

        if v * self.1 != self.0 {
            v += 1;
        }

        v
    }
}

impl FromStr for Fraction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split('.').collect();

        assert!(parts.len() <= 2);

        let mut upper = parts[0].parse::<i64>()?;

        let mut p = 1;

        if parts.len() == 2 {
            p = 10i64.pow(parts[1].len() as u32);

            upper *= p;

            upper += parts[1].parse::<i64>()?;
        }

        Ok(Self::new(upper, p))
    }
}

impl PartialOrd for Fraction {
    fn ge(
        &self,
        other: &Self,
    ) -> bool {
        !(self < other)
    }

    fn gt(
        &self,
        other: &Self,
    ) -> bool {
        !(self <= other)
    }

    fn le(
        &self,
        other: &Self,
    ) -> bool {
        self < other || self == other
    }

    fn lt(
        &self,
        other: &Self,
    ) -> bool {
        (*self - *other).0 < 0
    }

    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;

        Some(if self < other {
            Less
        } else if self == other {
            Equal
        } else {
            Greater
        })
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        let l = lcm(self.1, rhs.1);

        Self(l / self.1 * self.0 + l / rhs.1 * rhs.0, l)
    }
}

impl AddAssign for Fraction {
    fn add_assign(
        &mut self,
        rhs: Self,
    ) {
        *self = *self + rhs;
    }
}

impl Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self::Output { Self(-self.0, self.1) }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        self + -rhs
    }
}

impl SubAssign for Fraction {
    fn sub_assign(
        &mut self,
        rhs: Self,
    ) {
        *self = *self - rhs;
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(
        mut self,
        mut rhs: Self,
    ) -> Self::Output {
        let mut g = gcd(self.0, rhs.1);

        self.0 /= g;

        rhs.1 /= g;

        g = gcd(self.1, rhs.0);

        self.1 /= g;

        rhs.0 /= g;

        let mut res = Self(self.0 * rhs.0, self.1 * rhs.1);

        res.make_denominator_positive();

        res
    }
}

impl MulAssign for Fraction {
    fn mul_assign(
        &mut self,
        rhs: Self,
    ) {
        *self = *self * rhs;
    }
}

impl MulInv for Fraction {
    type Output = Self;

    fn mul_inv(mut self) -> Self::Output {
        use std::mem::swap;

        swap(&mut self.0, &mut self.1);

        self.make_denominator_positive();

        self
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(
        self,
        rhs: Self,
    ) -> Self::Output {
        self * rhs.mul_inv()
    }
}

impl DivAssign for Fraction {
    fn div_assign(
        &mut self,
        rhs: Self,
    ) {
        *self = *self / rhs;
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_new() {
        let cases = vec![
            ((1, 1), Fraction(1, 1)),
            ((10, 0), Fraction(1, 0)),
            ((-1, 0), Fraction(-1, 0)),
            ((3, -6), Fraction(-1, 2)),
        ];

        for ((upper, lower), correct) in cases {
            assert_eq!(Fraction::new(upper, lower), correct);
        }
    }

    #[test]

    fn test_from_str() {
        let cases = vec![
            ("100.000", Fraction(100, 1)),
            ("-100.000", Fraction(-100, 1)),
            ("123.45", Fraction(2469, 20)),
        ];

        for (s, correct) in cases {
            assert_eq!(s.parse::<Fraction>().unwrap(), correct);
        }
    }

    #[test]

    fn test_field_ops() {
        let a = Fraction::new(4, 5);

        let b = Fraction::new(-5, 8);

        assert_eq!(-a, Fraction(-4, 5));

        assert_eq!(a * b, Fraction(-1, 2));

        assert_eq!(a + b, Fraction(7, 40));

        assert_eq!(a / b, Fraction(-32, 25));

        assert_eq!(b / a, Fraction(-25, 32));

        assert!(a > b);
    }
}
