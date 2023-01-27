//! integer square root digit by digit recursive

pub fn isqrt(n: u64) -> u64 {
    if n < 2 {
        return n;
    }

    let x = isqrt(n >> 2) << 1;

    if (x + 1).pow(2) <= n {
        x + 1
    } else {
        x
    }
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
