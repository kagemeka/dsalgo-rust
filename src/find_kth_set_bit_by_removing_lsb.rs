/// k is 0-indexed
/// O(K)

pub fn kth_set_bit(
    mut n: u64,
    k: usize,
) -> usize {
    assert!(n.count_ones() as usize > k);

    for _ in 0..k {
        n &= n - 1;
    }

    n.trailing_zeros() as usize
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let n = 0b100101;

        assert_eq!(kth_set_bit(n, 0), 0);

        assert_eq!(kth_set_bit(n, 1), 2);

        assert_eq!(kth_set_bit(n, 2), 5);
    }
}
