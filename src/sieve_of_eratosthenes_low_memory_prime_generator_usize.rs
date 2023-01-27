use crate::{
    integer_square_root_with_binary_search_usize::isqrt,
    sieve_of_eratosthenes_enumerate_primes_in_range_query_usize::*,
};

pub struct PrimeGenerator {
    it: std::vec::IntoIter<usize>,
    range_sieve: EnumerateRangePrimes,
    lo_gen: std::iter::StepBy<std::ops::Range<usize>>,
    limit: usize,
    chunk_size: usize,
}

impl PrimeGenerator {
    /// lo <= hi < 10^14

    pub fn new(
        lo: usize,
        hi: usize,
    ) -> Self {
        let chunk_size = isqrt(hi) << 3;

        Self {
            it: vec![].into_iter(),
            range_sieve: EnumerateRangePrimes::new(hi),
            lo_gen: (lo..hi).step_by(chunk_size),
            limit: hi,
            chunk_size,
        }
    }
}

impl Iterator for PrimeGenerator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(p) = self.it.next() {
            return Some(p);
        }

        while let Some(lo) = self.lo_gen.next() {
            self.it = self
                .range_sieve
                .enumerate(lo, self.limit.min(lo + self.chunk_size))
                .into_iter();

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
