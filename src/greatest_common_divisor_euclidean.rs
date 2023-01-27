use std::ops::*;

pub fn gcd<T>(
    mut a: T,
    mut b: T,
) -> T
where
    T: Default + PartialEq + Copy + RemAssign,
{
    let zero = T::default();

    while b != zero {
        a %= b;

        std::mem::swap(&mut a, &mut b);
    }

    a
}

#[cfg(test)]

mod tests {

    use super::*;

    const CASES: &[(&[i32], i32)] = &[
        (&[], 0),
        (&[0], 0),
        (&[10], 10),
        (&[0, 2, 8, 4], 2),
        (&[33, -111], 3),
        (&[-33, 111], 3),
        (&[-33, -111], 3),
        (&[100, -3], 1),
        (&[-1, 0], 1),
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
