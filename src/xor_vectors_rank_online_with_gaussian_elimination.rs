/// online Xor Vectors Rank

pub struct XorVectorsRank {
    rank: usize,
    basis: Vec<usize>,
}

impl XorVectorsRank {
    pub fn new(n_bits: usize) -> Self {
        Self { rank: 0, basis: vec![0; n_bits] }
    }

    pub fn get_rank(&self) -> usize { self.rank }

    pub fn freedom_bits(&self) -> usize {
        let mut s: usize = 0;

        for (i, &b) in self.basis.iter().enumerate() {
            if b != 0 {
                s |= 1 << i;
            }
        }

        debug_assert_eq!(s.count_ones() as usize, self.rank);

        s
    }

    fn eliminate(
        &self,
        mut x: usize,
    ) -> usize {
        for (i, b) in self.basis.iter().enumerate() {
            if x >> i & 1 == 1 {
                x ^= b;
            }
        }

        x
    }

    fn normalize_basis(
        &mut self,
        i: usize,
    ) {
        let x = self.basis[i];

        for (j, b) in self.basis.iter_mut().enumerate() {
            if i != j && *b >> i & 1 == 1 {
                *b ^= x;
            }
        }
    }

    pub fn push(
        &mut self,
        mut x: usize,
    ) {
        x = self.eliminate(x);

        for (i, b) in self.basis.iter_mut().enumerate() {
            if x >> i & 1 == 0 {
                continue;
            }

            debug_assert!(*b == 0);

            *b = x;

            self.rank += 1;

            self.normalize_basis(i);

            break;
        }
    }

    pub fn is_independent(
        &self,
        x: usize,
    ) -> bool {
        self.eliminate(x) != 0
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut f = XorVectorsRank::new(4); // 0b0000
        assert_eq!(f.get_rank(), 0);

        f.push(0b1010);

        assert_eq!(f.get_rank(), 1);

        assert_eq!(f.freedom_bits(), 0b0010);

        f.push(0b1100);

        assert_eq!(f.freedom_bits(), 0b0110);

        assert_eq!(f.get_rank(), 2);

        f.push(0b0110);

        assert_eq!(f.freedom_bits(), 0b0110); // no change
        assert_eq!(f.get_rank(), 2);

        assert!(!f.is_independent(0b0110));
    }
}
