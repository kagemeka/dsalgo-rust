use crate::integer_square_root_with_binary_search_u64::isqrt;

pub fn ceil(n: u64) -> u64 {
    let x = isqrt(n);

    if x * x == n {
        x
    } else {
        x + 1
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
