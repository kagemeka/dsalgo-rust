use std::ops::*;

pub fn gcd<T>(
    mut a: T,
    mut b: T,
) -> T
where
    T: Default + PartialEq + Copy + PartialOrd + SubAssign,
{
    let zero = T::default();

    assert!(zero <= a && zero <= b);

    loop {
        if a < b {
            (a, b) = (b, a)
        }

        if b == zero {
            return a;
        }

        a -= b;
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    const CASES: &[(&[i32], i32)] = &[
        (&[], 0),
        (&[0], 0),
        (&[10], 10),
        (&[0, 2, 8, 4], 2),
        (&[10, 5], 5),
        (&[0, 10], 10),
    ];

    fn test_wrapper<F>(
        gcd: &F,
        a: i32,
        b: i32,
        expected: i32,
    ) where
        F: Fn(i32, i32) -> i32,
    {
        let mut g = gcd(a, b);

        if g < 0 {
            g = -g;
        }

        assert_eq!(g, expected);
    }

    #[test]

    fn test() {
        for &(v, ans) in CASES {
            if v.len() != 2 {
                continue;
            }

            test_wrapper(&gcd, v[0], v[1], ans);
        }
    }
}
