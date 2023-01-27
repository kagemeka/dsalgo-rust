//! integer square root with binary search

pub fn isqrt(n: usize) -> usize {
    if n < 2 {
        return n;
    }

    let mut ok = 0;

    let mut ng = n.min(1 << 32);

    while ng - ok > 1 {
        let x = (ok + ng) >> 1;

        if x * x <= n {
            ok = x;
        } else {
            ng = x;
        }
    }

    ok
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::integer_square_root_linear_u64::isqrt as naive;

        for i in 0..1000 {
            assert_eq!(isqrt(i), naive(i as u64) as usize);
        }

        let cases = vec![(std::usize::MAX, (1 << 32) - 1)];

        for (n, ans) in cases {
            assert_eq!(isqrt(n), ans);
        }
    }
}
