use crate::{
    integer_square_root_with_binary_search_u64::isqrt,
    sieve_of_eratosthenes_enumerate_primes_in_range_query_optim_u64::*,
};

pub struct PrimeGenerator {
    it: std::vec::IntoIter<u64>,
    range_sieve: EnumerateRangePrimes,
    ranges: std::vec::IntoIter<(u64, u64)>,
}

impl PrimeGenerator {
    /// [lo, hi)

    pub fn new(
        mut lo: u64,
        mut hi: u64,
    ) -> Self {
        if lo < 2 {
            lo = 2;
        }

        if hi < 2 {
            hi = 2;
        }

        let mut ranges = vec![];

        let size = (isqrt(hi) as usize) << 3;

        // 2 ~ 4
        // because range sieve has only odd numbers internally,
        // the size is sqrt / 2.
        // so we can check more than twice the range at once.
        // four times is best in test.
        for i in (lo..hi).step_by(size) {
            ranges.push((i, hi.min(i + size as u64)));
        }

        Self {
            it: vec![].into_iter(),
            range_sieve: EnumerateRangePrimes::new(hi as u64),
            ranges: ranges.into_iter(),
        }
    }
}

impl Iterator for PrimeGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(p) = self.it.next() {
            return Some(p);
        }

        while let Some((lo, hi)) = self.ranges.next() {
            self.it = self.range_sieve.enumerate(lo, hi).into_iter();

            if let Some(p) = self.it.next() {
                return Some(p);
            }
        }

        None
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut it = PrimeGenerator::new(20, 30);

        assert_eq!(it.next(), Some(23));

        assert_eq!(it.next(), Some(29));

        assert_eq!(it.next(), None);
    }
}
