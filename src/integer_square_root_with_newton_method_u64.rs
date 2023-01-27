//! integer square root with newton's method

pub fn isqrt(n: u64) -> u64 {
    let mut x0 = n >> 1;

    if x0 == 0 {
        return n;
    }

    loop {
        let x1 = (x0 + n / x0) >> 1;

        if x1 >= x0 {
            break;
        }

        x0 = x1;
    }

    x0
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::integer_square_root_with_binary_search_u64::isqrt as binary;

        for i in 0..1000 {
            assert_eq!(isqrt(i), binary(i));
        }
    }
}
